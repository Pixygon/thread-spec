# The Thread — an open spatial web

**The Thread is an open medium, not a product.** It is the successor to the web
where pages are *places* you walk into and links are *doorways* you step through.
This repository is its **standard**: the normative specifications, plus the JSON
Schema and the world corpus they are checked against. The conformance suite is a
separate installable tool (`thread-conformance`), so that the arbiter everyone
runs is one published artefact rather than a copy per checkout that quietly falls
behind. No single company owns the Thread — this repo is where the
format lives so that many browsers, many hosts, and many worlds can interoperate.

The promise the standard exists to keep:

> **Get a domain. Publish one JSON file. Anyone's browser can walk your world —
> with zero coordination with anyone.**

## Start here

- **[Protocol Overview](specs/thread-protocol-v0.1.md)** — how the pieces fit; read
  this first.
- **[World Manifest](specs/world-manifest-v0.1.md)** — the document format (the
  "HTML" of the Thread). The narrow waist every browser implements.
- **[Locator & Resolution](specs/locator-and-resolution-v0.1.md)** — addresses
  (`thread://…`) and zero-registry `.well-known` hosting.
- **[Behavior ABI](specs/behavior-abi-v0.1.md)** — sandboxed WASM interactivity
  (optional).
- **[Presence Wire](specs/presence-wire-v0.1.md)** — shared, multi-user worlds
  (optional).
- **[Passport](specs/passport-v0.1.md)** — one portable identity across every
  host, verified keys-only against the issuer's JWKS. Optional: a world MUST
  work for travelers who present none.
- **[Avatar](specs/avatar-v0.1.md)** — the portable "you" a Passport points at,
  so one saved avatar renders the same in every world (optional).
- **[Thread Markup](specs/thread-markup-v0.1.md)** — the authoring form. A
  `.thread` file reads like HTML + CSS and compiles to a World Manifest; hosts
  may serve it directly, and browsers accept either form (optional).
- **[Presence Topology](specs/presence-topology-v0.1.md)** — how presence is
  *organised*: federated relays, failover, area-of-interest scale, and a
  serverless P2P tier, so it survives any one operator disappearing.
- **[Weft](specs/weft-v0.1.md)** — the Thread's native code format:
  content-addressed, verified, total program graphs that cannot diverge,
  allocate unboundedly, or lie about what they are. With
  **[Weft Packages](specs/weft-pack-v0.1.md)**, how they are distributed.
- **[Structured Id](specs/structured-id-v0.1.md)** — `CCSSNNNN`, the name a
  world gives its own parts. Required by the manifest and, until now, defined
  only by a crate.
- **[Model](specs/model-v0.1.md)** — a model as a flat list of carving steps:
  the format a mesher reads and an agent can write a line at a time.
- **[Conformance Suite](specs/conformance-v0.1.md)** — the browser-independent
  check. **Passing it is what "conformant" means.**
- **[JSON Schema](schema/world-0.1.schema.json)** — the manifest's shape, for
  validating a `world.json` in any language. It checks shape only; the clauses
  that need the world's files or the network belong to the suite.

## The reference implementation

[**Pixygon/thread-engine**](https://github.com/Pixygon/thread-engine) implements
all of this in Rust and publishes it to crates.io — the engine, the language,
the mesher and the tools, MIT/Apache-2.0. It is *an* implementation, not the
standard: **where the two disagree, this repository is right and that one has a
bug.** It carries no copy of these documents, deliberately — a directory that
claims to mirror a spec and quietly falls behind it is worse than no copy at all.

```bash
cargo install thread-cli
thread init myplace && thread validate myplace && thread lint myplace
```

Contributing: **[CONTRIBUTING.md](CONTRIBUTING.md)** — anyone may propose a
change, and the rule that matters is that a spec change without a matching
conformance change is incomplete.

## Prove your world conforms

The conformance suite is the heart of the standard — it's how anyone verifies a
world (or an engine's output) honours the spec, with no reference-browser required.

```bash
# No Rust? Take a prebuilt binary — this installs `thread` and
# `thread-conformance` for Linux and macOS:
curl -fsSL https://raw.githubusercontent.com/Pixygon/thread-engine/main/install.sh | sh

# With Rust:
cargo install thread-conformance

# a local corpus (a directory of <name>/world.json)
thread-conformance worlds/

# a live host, over its .well-known
thread-conformance --live yourdomain.com

# a presence relay, or a P2P rendezvous
thread-conformance --relay wss://your-relay
thread-conformance --rendezvous wss://your-rendezvous
```

Green means conformant. That's the whole social contract: pass the suite and your
worlds interoperate with every Thread browser, forever.

## Publish a world in three steps

1. Author a `world.json` (see the [World Manifest](specs/world-manifest-v0.1.md)
   spec, or scaffold one with the `thread` CLI).
2. Serve it at `https://<yourdomain>/.well-known/thread/world.json` with
   `Content-Type: application/json` and `Access-Control-Allow-Origin: *`.
3. `thread-conformance --live <yourdomain>` → share `thread://<yourdomain>`.

## Implementations

- **Infinite** — the reference browser (Rust + wgpu), built on the Loom engine.
- **Loom** — an embeddable engine crate other browsers can build on (fill one
  `WorldLoader` seam with your renderer). It publishes to crates.io as
  **`thread-engine`** — "Loom" is its name in the source tree, not the name you
  install. Neither is privileged: implementing the specs directly in your own
  engine is equally valid.

The specs are the authority. Where an implementation and a spec disagree, the spec
wins — file an issue.

## Governance & versioning

See **[GOVERNANCE.md](GOVERNANCE.md)**. In short: `thread/<major>.<minor>` version
tags; additive-within-major; the conformance suite is versioned with the specs;
changes happen in the open against this repo.

## License

Specifications: [CC-BY-4.0](LICENSE-SPEC). Code — the schema here, and the
conformance suite and tools that live in
[thread-engine](https://github.com/Pixygon/thread-engine):
[MIT OR Apache-2.0](LICENSE-CODE). Both permissive on purpose — copy, implement,
fork, compete.
