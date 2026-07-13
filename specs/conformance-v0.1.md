# Thread Conformance Suite v0.1

A standard is only real if it can be checked **without the reference browser**.
This is that check: a browser-independent suite that verifies a corpus of worlds
honours the Thread spec. Anyone can run it against their own worlds — or their own
engine's output — and prove conformance with zero contact.

The reference implementation is the `thread-conformance` crate
(`crates/thread-conformance`), a library + CLI. It is designed to move, unchanged,
into the neutral `thread-spec` repository so the suite lives apart from any one
browser.

## Running it

```bash
# validate a local corpus (a directory of <name>/world.json)
cargo run -p thread-conformance -- worlds
thread-conformance path/to/worlds

# check a LIVE host over its .well-known (host, host/path, or thread://…)
thread-conformance --live yourdomain.com
thread-conformance --live yourdomain.com/gallery

# probe a presence RELAY's wire format (Passport via INFINITE_PASSPORT)
thread-conformance --relay wss://relay.example.com/thread/<worldId>
```

Same clauses, same pass rule, whether the worlds are on disk, on the open web, or a
live relay — one tool.

Exit code is `0` when **conformant** (no Error-severity clause failed) and non-zero
otherwise. As a library:

```rust
let corpus = thread_conformance::load_corpus(root);
let report = thread_conformance::run(&corpus);
assert!(report.passed());
```

## Severity

- **Error** — a real spec violation. A corpus with any failed Error clause is
  **non-conformant**.
- **Warn** — a quality signal that does not break interop (e.g. an unlinked world).
  Reported, but the suite still passes.

## Clauses (v0.1)

| # | Clause | Severity | Passes when |
|---|--------|----------|-------------|
| C1 | world manifests validate | Error | every `world.json` parses and validates through the reference manifest implementation (`thread/…` tag, references resolve, portals address the Thread) |
| C2 | worlds declare a spawn | Error | every world has at least one arrival point (`spawns[0]`) |
| C3 | portal destinations are valid Locators | Error | every portal `to` parses as `thread://…` |
| C4 | constellation is connected | Warn | from an entry world (a world named `nexus`, else the veil-richest hub), every other world in the corpus is reachable via **internal** veils. External veils (to worlds hosted elsewhere) are counted, never penalized |
| C5 | veils carry labels | Warn | every portal has a non-empty `label` (so a browser can name the doorway) |

### Transport clauses (`--live` only)

When checking a live host, the served response must also honour the transport
contract from [locator-and-resolution-v0.1](locator-and-resolution-v0.1.md) before
the manifest clauses run:

| Clause | Severity | Passes when |
|--------|----------|-------------|
| reachable over HTTPS | Error | the `.well-known` GET returns 2xx |
| serves any-origin CORS | Error | `Access-Control-Allow-Origin: *` (so a cross-origin web browser can fetch it) |
| declares JSON content-type | Warn | `Content-Type` contains `json` (the manifest still parses without it) |

The served body is then run through clauses C1–C5 as a one-world corpus, so a host
proves itself with exactly the checks a browser would apply.

### Relay clauses (`--relay` only)

A presence relay is probed against [presence-wire-v0.1](presence-wire-v0.1.md): the
suite connects (`wss://…`), sends `join`, and validates the messages it receives.
The message validators are pure and unit-tested, so the wire rules hold independent
of any relay:

| Message | Clause | Severity |
|---------|--------|----------|
| `welcome` | tagged `t=welcome`; assigns an occupant `id`; lists `occupants` | Error |
| `welcome` | declares `tick_hz` | Warn |
| `pose` | tagged `t=pose`; carries `id`; **server-stamped `ts`**; position `p[3]`; **velocity `v[3]`**; an orientation (`r` xyzw *or* yaw `y`) | Error |
| `pose` | orientation is a unit quaternion; carries animation state `a` | Warn |

`ts` and `v` are Errors because they are exactly what a client needs to interpolate
~100 ms in the past without trusting peer clocks (§4). With a lone probe client a
relay may not fan a pose back (area-of-interest / no peers); that's reported as a
note, not a failure.

### Resolution model

A portal destination is **internal** to a corpus when its Locator resolves to a
world present in that corpus — i.e. the Locator's path (or host, when path-less)
matches a world's directory name. This mirrors the local resolver's mapping
(`thread://host/path` → `<root>/<path>/world.json`). Everything else is **external**
and is a legitimate cross-host link, not a failure.

## Scope of v0.1 and what's next

v0.1 checks the **static corpus**, the **live transport contract** (HTTPS + CORS +
content-type), and a **presence relay's wire format**. Related, engine-side:

- **Scripted walk** — [`loom::walk`] drives the reference engine through a recorded
  path and asserts the worlds it lands in (dogfoods the engine as an oracle; lives
  with the engine, not this suite, to keep the suite engine-independent).

See also: [world-manifest-v0.1](world-manifest-v0.1.md),
[locator-and-resolution-v0.1](locator-and-resolution-v0.1.md).
