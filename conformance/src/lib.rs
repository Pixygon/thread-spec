//! # The Thread conformance suite
//!
//! A standard is only real if it can be checked *without* the reference browser.
//! This crate is that check: point it at a corpus of worlds (a directory of
//! `world.json`s) and it reports, browser-independently, whether they honour the
//! spec — manifests validate, worlds are enterable, links are well-formed, and the
//! constellation hangs together. It's the artifact that lets a third party prove
//! their worlds (or their own engine's output) conform, with zero contact.
//!
//! Clauses are either **Error** (a real conformance violation — fails the suite)
//! or **Warn** (a quality signal that doesn't break interop — reported, but the
//! suite still passes). [`run`] returns a [`Report`]; [`Report::passed`] is true
//! when no Error clause failed.

use std::path::{Path, PathBuf};

use infinite_manifest::{Locator, WorldManifest};

pub mod relay;

/// Whether a failed clause breaks conformance or is merely a quality signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// A real spec violation — fails the suite.
    Error,
    /// A quality signal — reported, but interop still holds.
    Warn,
}

/// The outcome of one conformance clause.
#[derive(Debug, Clone)]
pub struct Clause {
    pub name: &'static str,
    pub severity: Severity,
    pub pass: bool,
    /// Human-readable specifics (offending worlds, external links, orphans…).
    pub notes: Vec<String>,
}

/// The full conformance report over a corpus.
#[derive(Debug, Clone)]
pub struct Report {
    pub clauses: Vec<Clause>,
    /// How many worlds loaded and validated.
    pub worlds: usize,
}

impl Report {
    /// The suite passes when no **Error**-severity clause failed. Failed **Warn**
    /// clauses are reported but do not break conformance.
    pub fn passed(&self) -> bool {
        self.clauses
            .iter()
            .all(|c| c.pass || c.severity == Severity::Warn)
    }
}

/// A world that loaded and validated.
pub struct World {
    /// The world's key in the corpus — its directory path under the root, which is
    /// exactly what a Locator's path (or host) resolves to.
    pub name: String,
    pub manifest: WorldManifest,
}

/// A world that failed to load or validate.
pub struct LoadError {
    pub name: String,
    pub error: String,
}

/// A loaded corpus: the worlds that validated, plus the ones that didn't.
pub struct Corpus {
    pub worlds: Vec<World>,
    pub load_errors: Vec<LoadError>,
}

/// Load every `world.json` under `root` (one per immediate subdirectory), parsing
/// and validating each through the reference manifest implementation.
pub fn load_corpus(root: &Path) -> Corpus {
    let mut worlds = Vec::new();
    let mut load_errors = Vec::new();

    let mut entries: Vec<PathBuf> = match std::fs::read_dir(root) {
        Ok(rd) => rd.flatten().map(|e| e.path()).collect(),
        Err(e) => {
            load_errors.push(LoadError { name: root.display().to_string(), error: e.to_string() });
            return Corpus { worlds, load_errors };
        }
    };
    entries.sort();

    for dir in entries {
        let wj = dir.join("world.json");
        if !wj.exists() {
            continue;
        }
        let name = dir.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
        match std::fs::read_to_string(&wj) {
            Ok(text) => match WorldManifest::from_json(&text) {
                Ok(manifest) => worlds.push(World { name, manifest }),
                Err(e) => load_errors.push(LoadError { name, error: e.to_string() }),
            },
            Err(e) => load_errors.push(LoadError { name, error: e.to_string() }),
        }
    }
    Corpus { worlds, load_errors }
}

/// Build a one-world corpus from a single manifest's text (the live-host path:
/// fetch a `thread://host` and run the spec clauses on what it served). `name` is
/// the world's corpus key (its Locator path/host).
pub fn single_corpus(name: &str, manifest_text: &str) -> Corpus {
    match WorldManifest::from_json(manifest_text) {
        Ok(manifest) => Corpus {
            worlds: vec![World { name: name.to_string(), manifest }],
            load_errors: Vec::new(),
        },
        Err(e) => Corpus {
            worlds: Vec::new(),
            load_errors: vec![LoadError { name: name.to_string(), error: e.to_string() }],
        },
    }
}

/// The transport contract a live host must satisfy so *any* browser — including a
/// cross-origin web one — can walk it. Pure over the response facts, so it's
/// testable without a network. `reachable` is whether the GET completed with a 2xx.
pub fn transport_clauses(reachable: bool, status: u16, content_type: &str, cors: &str) -> Vec<Clause> {
    vec![
        Clause {
            name: "reachable over HTTPS",
            severity: Severity::Error,
            pass: reachable,
            notes: if reachable { vec![] } else { vec![format!("HTTP {status}")] },
        },
        Clause {
            name: "serves any-origin CORS",
            severity: Severity::Error,
            pass: cors == "*",
            notes: if cors == "*" {
                vec![]
            } else if cors.is_empty() {
                vec!["missing Access-Control-Allow-Origin (a web browser can't fetch it)".into()]
            } else {
                vec![format!("Access-Control-Allow-Origin: {cors} (not '*')")]
            },
        },
        Clause {
            name: "declares JSON content-type",
            severity: Severity::Warn,
            pass: content_type.contains("json"),
            notes: if content_type.contains("json") {
                vec![]
            } else {
                vec![format!("Content-Type: '{content_type}' (expected application/json)")]
            },
        },
    ]
}

/// Whether a slice of clauses passes (no failed Error clause).
pub fn clauses_pass(clauses: &[Clause]) -> bool {
    clauses.iter().all(|c| c.pass || c.severity == Severity::Warn)
}

/// The corpus key a Locator resolves to — its path, or its host when path-less
/// (mirrors the local resolver: `thread://host/path` → `<root>/<path>`).
fn dest_key(loc: &Locator) -> &str {
    if loc.path.is_empty() {
        &loc.host
    } else {
        &loc.path
    }
}

/// Run the full suite over a loaded corpus.
pub fn run(corpus: &Corpus) -> Report {
    let mut clauses = Vec::new();
    let names: std::collections::HashSet<&str> =
        corpus.worlds.iter().map(|w| w.name.as_str()).collect();

    // C1 — every world manifest validates. (Error)
    clauses.push(Clause {
        name: "world manifests validate",
        severity: Severity::Error,
        pass: corpus.load_errors.is_empty(),
        notes: corpus
            .load_errors
            .iter()
            .map(|e| format!("{}: {}", e.name, e.error))
            .collect(),
    });

    // C2 — every world declares at least one spawn (an arrival point). (Error)
    let no_spawn: Vec<String> = corpus
        .worlds
        .iter()
        .filter(|w| w.manifest.spawns.is_empty())
        .map(|w| w.name.clone())
        .collect();
    clauses.push(Clause {
        name: "worlds declare a spawn",
        severity: Severity::Error,
        pass: no_spawn.is_empty(),
        notes: no_spawn.iter().map(|n| format!("{n}: no spawns")).collect(),
    });

    // C3 — every portal destination is a well-formed Locator. (Error)
    let mut bad_locators = Vec::new();
    for w in &corpus.worlds {
        for p in &w.manifest.portals {
            if Locator::parse(&p.to).is_none() {
                bad_locators.push(format!("{}: portal '{}' → '{}'", w.name, p.id, p.to));
            }
        }
    }
    clauses.push(Clause {
        name: "portal destinations are valid Locators",
        severity: Severity::Error,
        pass: bad_locators.is_empty(),
        notes: bad_locators,
    });

    // C4 — the constellation is connected: from an entry world, every other world
    // is reachable via internal veils. External links (to worlds hosted elsewhere)
    // are fine and simply aren't traversed. Orphans are a quality signal. (Warn)
    let (entry, orphans, internal, external) = analyze_graph(&corpus.worlds, &names);
    let mut notes = vec![format!(
        "entry '{}' · {} internal link(s) · {} external link(s)",
        entry.as_deref().unwrap_or("—"),
        internal,
        external
    )];
    notes.extend(orphans.iter().map(|o| format!("unreachable from entry: {o}")));
    clauses.push(Clause {
        name: "constellation is connected",
        severity: Severity::Warn,
        pass: orphans.is_empty(),
        notes,
    });

    // C5 — veils carry a human label (so a browser can name the doorway). (Warn)
    let mut unlabeled = Vec::new();
    for w in &corpus.worlds {
        for p in &w.manifest.portals {
            if p.label.trim().is_empty() {
                unlabeled.push(format!("{}: portal '{}' has no label", w.name, p.id));
            }
        }
    }
    clauses.push(Clause {
        name: "veils carry labels",
        severity: Severity::Warn,
        pass: unlabeled.is_empty(),
        notes: unlabeled,
    });

    Report { clauses, worlds: corpus.worlds.len() }
}

/// Pick an entry world, BFS the internal veil graph, and return
/// `(entry, orphans, internal_link_count, external_link_count)`.
fn analyze_graph(
    worlds: &[World],
    names: &std::collections::HashSet<&str>,
) -> (Option<String>, Vec<String>, usize, usize) {
    if worlds.is_empty() {
        return (None, Vec::new(), 0, 0);
    }

    // Entry: prefer a world named "nexus"; else the hub with the most veils.
    let entry = worlds
        .iter()
        .find(|w| w.name == "nexus")
        .or_else(|| worlds.iter().max_by_key(|w| w.manifest.portals.len()))
        .map(|w| w.name.clone());

    // Adjacency over internal links; tally internal vs external.
    let mut internal = 0usize;
    let mut external = 0usize;
    let mut adj: std::collections::HashMap<&str, Vec<String>> = std::collections::HashMap::new();
    for w in worlds {
        for p in &w.manifest.portals {
            match Locator::parse(&p.to) {
                Some(loc) if names.contains(dest_key(&loc)) => {
                    internal += 1;
                    adj.entry(w.name.as_str()).or_default().push(dest_key(&loc).to_string());
                }
                _ => external += 1,
            }
        }
    }

    // BFS from the entry.
    let mut seen = std::collections::HashSet::new();
    if let Some(start) = &entry {
        let mut queue = vec![start.clone()];
        seen.insert(start.clone());
        while let Some(cur) = queue.pop() {
            if let Some(nexts) = adj.get(cur.as_str()) {
                for n in nexts {
                    if seen.insert(n.clone()) {
                        queue.push(n.clone());
                    }
                }
            }
        }
    }

    let mut orphans: Vec<String> = worlds
        .iter()
        .map(|w| w.name.clone())
        .filter(|n| !seen.contains(n))
        .collect();
    orphans.sort();
    (entry, orphans, internal, external)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn corpus_from(worlds: Vec<(&str, WorldManifest)>) -> Corpus {
        Corpus {
            worlds: worlds.into_iter().map(|(n, m)| World { name: n.into(), manifest: m }).collect(),
            load_errors: Vec::new(),
        }
    }

    fn world(id: &str, spawns: bool, portals: &[(&str, &str, &str)]) -> WorldManifest {
        let spawn_json = if spawns {
            r#""spawns": [{ "name": "entry", "position": [0,0,0] }],"#
        } else {
            ""
        };
        let portal_json: Vec<String> = portals
            .iter()
            .map(|(pid, to, label)| {
                format!(r#"{{ "id": "{pid}", "position": [0,0,0], "to": "{to}", "label": "{label}" }}"#)
            })
            .collect();
        let text = format!(
            r#"{{ "thread": "thread/0.1", "world": {{ "id": "{id}", "title": "{id}" }},
                {spawn_json} "portals": [{}] }}"#,
            portal_json.join(",")
        );
        WorldManifest::from_json(&text).unwrap()
    }

    #[test]
    fn a_connected_labeled_corpus_passes_cleanly() {
        let c = corpus_from(vec![
            ("nexus", world("nexus", true, &[("to-a", "thread://a.io/alpha", "Alpha")])),
            ("alpha", world("alpha", true, &[("to-n", "thread://x.io/nexus", "Nexus")])),
        ]);
        let r = run(&c);
        assert!(r.passed(), "no error clauses should fail");
        assert!(r.clauses.iter().all(|cl| cl.pass), "and no warnings either: {:?}", r.clauses);
    }

    #[test]
    fn a_missing_spawn_is_an_error_and_fails_the_suite() {
        let c = corpus_from(vec![("alpha", world("alpha", false, &[]))]);
        let r = run(&c);
        assert!(!r.passed());
        let spawn_clause = r.clauses.iter().find(|c| c.name == "worlds declare a spawn").unwrap();
        assert!(!spawn_clause.pass);
        assert_eq!(spawn_clause.severity, Severity::Error);
    }

    #[test]
    fn an_orphan_world_warns_but_still_passes() {
        // 'lonely' is in the corpus but nothing links to it.
        let c = corpus_from(vec![
            ("nexus", world("nexus", true, &[("to-a", "thread://a.io/alpha", "Alpha")])),
            ("alpha", world("alpha", true, &[])),
            ("lonely", world("lonely", true, &[])),
        ]);
        let r = run(&c);
        assert!(r.passed(), "an orphan is a warning, not a conformance failure");
        let conn = r.clauses.iter().find(|c| c.name == "constellation is connected").unwrap();
        assert!(!conn.pass);
        assert_eq!(conn.severity, Severity::Warn);
        assert!(conn.notes.iter().any(|n| n.contains("lonely")));
    }

    /// The repo's shipped corpus must have zero **Error**-severity failures — the
    /// suite guarding the worlds we host. (Warn clauses like orphan worlds are
    /// allowed; this asserts real conformance, not full connectivity.)
    #[test]
    fn the_shipped_worlds_corpus_is_conformant() {
        // In thread-spec the fixture corpus is a sibling of conformance/ at the
        // repo root (not ../../ as in Infinite's crates/ layout).
        let root = format!("{}/../worlds", env!("CARGO_MANIFEST_DIR"));
        let corpus = load_corpus(std::path::Path::new(&root));
        let report = run(&corpus);
        assert!(report.worlds >= 8, "expected the hosted constellation, found {}", report.worlds);
        for c in &report.clauses {
            if c.severity == Severity::Error {
                assert!(c.pass, "error clause '{}' failed: {:?}", c.name, c.notes);
            }
        }
        assert!(report.passed());
    }

    #[test]
    fn transport_contract_flags_missing_cors_as_error() {
        let ok = transport_clauses(true, 200, "application/json", "*");
        assert!(clauses_pass(&ok));

        let no_cors = transport_clauses(true, 200, "application/json", "");
        assert!(!clauses_pass(&no_cors), "missing CORS breaks cross-origin browsers");
        let cors_clause = no_cors.iter().find(|c| c.name == "serves any-origin CORS").unwrap();
        assert_eq!(cors_clause.severity, Severity::Error);

        // Wrong content-type is only a warning — the manifest still parses.
        let text_plain = transport_clauses(true, 200, "text/plain", "*");
        assert!(clauses_pass(&text_plain));
        assert!(!text_plain.iter().find(|c| c.name == "declares JSON content-type").unwrap().pass);
    }

    #[test]
    fn single_corpus_runs_the_spec_clauses_on_one_fetched_world() {
        let text = r#"{ "thread": "thread/0.1", "world": { "id": "w", "title": "Live" },
            "spawns": [{ "name": "entry", "position": [0,0,0] }],
            "portals": [{ "id": "p", "position": [0,0,0], "to": "thread://x.io/y", "label": "Y" }] }"#;
        let corpus = single_corpus("myhost.com", text);
        let report = run(&corpus);
        assert_eq!(report.worlds, 1);
        assert!(report.passed());

        // A malformed live manifest surfaces as a C1 error.
        let bad = single_corpus("myhost.com", "{ nope }");
        assert!(!run(&bad).passed());
    }

    #[test]
    fn external_links_are_counted_not_penalized() {
        let c = corpus_from(vec![(
            "nexus",
            world("nexus", true, &[("out", "thread://someone-elses-host.com/room", "Elsewhere")]),
        )]);
        let r = run(&c);
        assert!(r.passed());
        let conn = r.clauses.iter().find(|c| c.name == "constellation is connected").unwrap();
        assert!(conn.notes.iter().any(|n| n.contains("1 external link")));
    }
}
