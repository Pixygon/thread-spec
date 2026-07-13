//! `thread-conformance` — run the Thread conformance suite, local, live, or relay.
//!
//!   thread-conformance [worlds-root]        check a local corpus (default ./worlds)
//!   thread-conformance --live <host>        fetch a live host and check what it serves
//!   thread-conformance --relay <wss-url>    probe a presence relay's wire format
//!
//! `<host>` may be a bare domain, `host/path`, or a full `thread://…` Locator.
//! `<wss-url>` is `wss://<relay>/thread/<worldId>`; pass a Passport via the
//! `INFINITE_PASSPORT` env var (the relay verifies it on join).
//! Exits non-zero if any **Error**-severity clause fails (Warn clauses are shown
//! but don't fail it) — the same rule everywhere.

use std::path::PathBuf;
use std::process::ExitCode;

use infinite_manifest::{well_known_url, Locator};
use thread_conformance::{
    clauses_pass, load_corpus, run, single_corpus, transport_clauses, Clause, Severity,
};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--live") => match args.get(1) {
            Some(target) => run_live(target),
            None => {
                eprintln!("usage: thread-conformance --live <host | host/path | thread://…>");
                ExitCode::FAILURE
            }
        },
        Some("--relay") => match args.get(1) {
            Some(url) => run_relay(url),
            None => {
                eprintln!("usage: thread-conformance --relay wss://<relay>/thread/<worldId>");
                ExitCode::FAILURE
            }
        },
        _ => run_local(args.first().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("worlds"))),
    }
}

/// Print a clause and fold its pass/fail into `ok` (Warn failures don't flip it).
fn print_clauses(clauses: &[Clause]) {
    for c in clauses {
        let mark = if c.pass { "✓" } else { "✗" };
        let sev = match c.severity {
            Severity::Error => "error",
            Severity::Warn => "warn ",
        };
        println!("  {mark} [{sev}] {}", c.name);
        for note in &c.notes {
            println!("        · {note}");
        }
    }
}

fn run_local(root: PathBuf) -> ExitCode {
    println!("Thread conformance · corpus: {}\n", root.display());
    let corpus = load_corpus(&root);
    let report = run(&corpus);
    println!("{} world(s) loaded\n", report.worlds);
    print_clauses(&report.clauses);
    println!();
    if report.passed() {
        println!("✓ CONFORMANT — {} world(s) honour the Thread spec.", report.worlds);
        ExitCode::SUCCESS
    } else {
        println!("✗ NON-CONFORMANT — fix the ✗ [error] clauses above.");
        ExitCode::FAILURE
    }
}

fn run_live(target: &str) -> ExitCode {
    let (host, path) = split_target(target);
    let url = well_known_url(&host, &path);
    println!("Thread conformance · live: thread://{host}{}  →  {url}\n",
        if path.is_empty() { String::new() } else { format!("/{path}") });

    let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
    let fetched = rt.block_on(fetch(&url));

    // Transport contract (HTTPS reachable, any-origin CORS, JSON content-type).
    let transport = transport_clauses(fetched.reachable, fetched.status, &fetched.content_type, &fetched.cors);
    println!("transport:");
    print_clauses(&transport);

    // Spec clauses on the served manifest.
    let corpus_key = if path.is_empty() { host.clone() } else { path.clone() };
    let corpus = single_corpus(&corpus_key, &fetched.body);
    let report = run(&corpus);
    println!("\nmanifest:");
    print_clauses(&report.clauses);
    println!();

    if clauses_pass(&transport) && report.passed() {
        println!("✓ CONFORMANT — thread://{host} is live and honours the spec. Walkable anywhere.");
        ExitCode::SUCCESS
    } else {
        println!("✗ NON-CONFORMANT — fix the ✗ [error] clauses above.");
        ExitCode::FAILURE
    }
}

fn run_relay(url: &str) -> ExitCode {
    let passport = std::env::var("INFINITE_PASSPORT").unwrap_or_default();
    println!("Thread conformance · relay: {url}\n");
    if passport.is_empty() {
        println!("(no INFINITE_PASSPORT set — the relay may reject the join)\n");
    }

    let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
    let outcome = rt.block_on(thread_conformance::relay::probe(url, &passport, 5000));

    print_clauses(&outcome.clauses);
    if !outcome.notes.is_empty() {
        println!();
        for n in &outcome.notes {
            println!("  · {n}");
        }
    }
    println!();

    if clauses_pass(&outcome.clauses) {
        println!("✓ CONFORMANT — the relay speaks presence-wire-v0.1.");
        ExitCode::SUCCESS
    } else {
        println!("✗ NON-CONFORMANT — fix the ✗ [error] clauses above.");
        ExitCode::FAILURE
    }
}

struct Fetched {
    reachable: bool,
    status: u16,
    content_type: String,
    cors: String,
    body: String,
}

async fn fetch(url: &str) -> Fetched {
    match reqwest::get(url).await {
        Ok(resp) => {
            let status = resp.status();
            let headers = resp.headers().clone();
            let header = |k: &str| headers.get(k).and_then(|v| v.to_str().ok()).unwrap_or("").to_string();
            let content_type = header("content-type");
            let cors = header("access-control-allow-origin");
            let body = resp.text().await.unwrap_or_default();
            Fetched { reachable: status.is_success(), status: status.as_u16(), content_type, cors, body }
        }
        Err(e) => Fetched {
            reachable: false,
            status: 0,
            content_type: String::new(),
            cors: String::new(),
            body: format!("(fetch failed: {e})"),
        },
    }
}

/// Split a live target into `(host, path)` — accepts bare host, `host/path`, or a
/// `thread://…` Locator.
fn split_target(target: &str) -> (String, String) {
    if let Some(loc) = Locator::parse(target) {
        return (loc.host, loc.path);
    }
    let t = target.trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/');
    match t.split_once('/') {
        Some((h, p)) => (h.to_string(), p.to_string()),
        None => (t.to_string(), String::new()),
    }
}
