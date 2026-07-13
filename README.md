# The Thread — an open spatial web

**The Thread is an open medium, not a product.** It is the successor to the web
where pages are *places* you walk into and links are *doorways* you step through.
This repository is its **standard**: the normative specifications plus a runnable
conformance suite. No single company owns the Thread — this repo is where the
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
- **[Conformance Suite](specs/conformance-v0.1.md)** — the browser-independent
  check. **Passing it is what "conformant" means.**

## Prove your world conforms

The conformance suite is the heart of the standard — it's how anyone verifies a
world (or an engine's output) honours the spec, with no reference-browser required.

```bash
# a local corpus (a directory of <name>/world.json)
thread-conformance worlds/

# a live host, over its .well-known
thread-conformance --live yourdomain.com
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

- **Infinite** — the reference browser (Rust + wgpu), built on the **Loom** engine.
- **Loom** — an embeddable engine crate other browsers can build on (fill one
  `WorldLoader` seam with your renderer). Neither is privileged: implementing the
  specs directly in your own engine is equally valid.

The specs are the authority. Where an implementation and a spec disagree, the spec
wins — file an issue.

## Governance & versioning

See **[GOVERNANCE.md](GOVERNANCE.md)**. In short: `thread/<major>.<minor>` version
tags; additive-within-major; the conformance suite is versioned with the specs;
changes happen in the open against this repo.

## License

Specifications: [CC-BY-4.0](LICENSE-SPEC). Conformance suite (code):
[MIT OR Apache-2.0](LICENSE-CODE). Both permissive on purpose — copy, implement,
fork, compete.
