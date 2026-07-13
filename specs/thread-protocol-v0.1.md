# The Thread — Protocol Overview v0.1

The Thread is an open, spatial, present, time-aware medium — the successor to the
web where **pages are places** and **links are doorways you step through**. This
document is the map: it explains how the pieces fit, so you can read the five
normative specs in the right order and build either side — a **world** anyone can
walk into, or a **browser** that walks them.

The one rule the whole design serves: **anyone can publish a world on their own
domain, and anyone's browser can walk it, with zero coordination.** No registry,
no account, no gatekeeper. Everything below exists to make that true.

## The layers (the "narrow waist")

Like the web, the Thread has a narrow waist — one document format every browser
implements, or nothing interoperates. Around it sit addressing, behavior,
presence, and a conformance check.

```
        ┌───────────────────────────────────────────────┐
        │  Browsers        Infinite · yours · a web view │   (many, competing)
        ├───────────────────────────────────────────────┤
        │  Presence Wire   shared, multi-user worlds     │   (optional upgrade)
        │  Behavior ABI    sandboxed WASM interactivity  │   (optional)
        ├───────────────────────────────────────────────┤
        │  WORLD MANIFEST  the document format  ← waist  │   (everyone implements)
        ├───────────────────────────────────────────────┤
        │  Locator &       thread://host/world@when#place│
        │  Resolution      + .well-known hosting         │   (addressing / "DNS")
        └───────────────────────────────────────────────┘
```

A world is **static-first, presence-optional**: at minimum it's a single JSON file
on a static host. Presence, behaviors, and time are upgrades you opt into.

## The five specs, in reading order

1. **[World Manifest](world-manifest-v0.1.md)** — the "HTML" of the Thread. A
   renderer-agnostic JSON description of a place: prefabs (glTF or built-in meshes
   + a standard PBR material model), placements, portals, spawns, and optional
   behaviors/presence. This is the waist — start here.

2. **[Locator & Resolution](locator-and-resolution-v0.1.md)** — the address of a
   place and how a browser finds it. `thread://host/world@when#place`, resolved
   registry → the host's own **`.well-known/thread/world.json`** → local. The
   `.well-known` convention is what makes zero-coordination hosting real: HTTPS +
   `application/json` + `Access-Control-Allow-Origin: *`, and any browser can reach
   your world.

3. **[Behavior ABI](behavior-abi-v0.1.md)** — how a world is interactive without
   trusting its code. Sandboxed WASM modules receive events and return declarative
   Actions (open a Codex entry, buy, navigate, emit presence, set state); the
   browser — never the module — performs them. Optional.

4. **[Presence Wire](presence-wire-v0.1.md)** — how a solo world becomes a shared
   one. A world names a relay; browsers exchange poses over it (RH, +Y-up, metres;
   quaternion xyzw; interpolated ~100 ms in the past). Absent a relay, the world
   gracefully degrades to solo. Optional. Its companion,
   **[Presence Topology](presence-topology-v0.1.md)**, fixes *how presence is
   organized* — federated relays, failover, area-of-interest scale, and a
   serverless P2P tier — so presence survives any single operator disappearing.

5. **[Conformance Suite](conformance-v0.1.md)** — the browser-independent check
   that a corpus of worlds (or a live host) honours all of the above. **The suite
   is the real standard**: if your worlds pass it, they are conformant, no matter
   whose tools made them.

## Addressing, concretely

```
thread://market.pixygon.io/shops/bazaar@1998#entry
        └──────┬───────┘ └────┬────┘ └─┬─┘ └─┬─┘
             host          world path  when  place
```

- **host** — the authority (a domain, like the web's).
- **world path** — the world on that host (`.well-known/thread/<path>/world.json`).
- **@when** — an OPTIONAL timeline year; time is a first-class navigation axis.
- **#place** — an OPTIONAL named spawn/portal to arrive at.

A **portal** in one world names another world's Locator as its `to`. Following it
is **veilwalking** — the hyperlink traversal of the Thread. Portals may point to
other hosts; that cross-host link, resolved by each host's own `.well-known`, is
the web-of-worlds.

## Building a browser

A browser must, at minimum: parse + validate a World Manifest, resolve Locators
(at least `.well-known` + local), render the manifest's prefabs/placements, and
veilwalk portals. Presence, behaviors, and `@when` are progressive enhancements.

The reference browser, **Infinite**, is built on **Loom** — an embeddable engine
crate that does resolution, world-loading, veilwalking, Codex lookups, the in-world
web reader, and presence, behind a `WorldLoader` seam. Loom is one way to build a
browser (fill the seam with your renderer); implementing the specs directly in your
own engine is equally valid. Neither is privileged — the specs are.

## Versioning

Every manifest carries a `thread/<major>.<minor>` tag. Within a major version,
additions are backward-compatible: an older browser ignores fields it doesn't know;
a newer browser supplies defaults for fields an older world omits. The conformance
suite is versioned alongside the specs. See the standard's `GOVERNANCE.md` for how
versions advance.

## Status

v0.1 is an **early, implemented** draft: the World Manifest, Locator/Resolution
(incl. `.well-known`), and the conformance suite (static + live) are built and in
use; Behavior ABI and Presence Wire are specified and partially implemented. The
normative home for these documents is the neutral `thread-spec` repository; this
copy travels with the reference browser.
