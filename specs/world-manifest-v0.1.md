# World Manifest — v0.1

**Status:** Draft · **Layer:** Content (the "HTML" of the Thread) · **Reference impl:** [`infinite-manifest`](../../crates/infinite-manifest)

## 1. What the Thread is

The **Thread** is an open, spatial, present, time-aware successor to the web.
Where the web serves *documents* linked by *hyperlinks*, the Thread serves
**worlds** — persistent, shared *places* — linked by **portals** (veils you step
through). Worlds are *pearls*; the Thread strings them together (see the canon
entry `the-pearl-dress`).

The Thread is a **standard**, not a product. Infinite is its first *browser* (its
Mosaic); anyone may build another. This document specifies the **World
Manifest** — the open, renderer-agnostic document every browser must be able to
resolve and render. It is the narrow waist of the stack: implement it, or nothing
interoperates.

### The stack (for context)

| Layer | The Thread | Web analog |
|-------|-----------|------------|
| Address | **Locator** (`thread://…`) | URL |
| Content | **World Manifest** (this doc) | HTML |
| Transfer | Arrival (fetch manifest + stream assets over HTTPS/CDN) | HTTP |
| Presence | shared avatar/state sync via a relay | *(none)* |
| Identity | portable Passport (DID-style) | cookies/accounts |
| Behavior | sandboxed WASM modules | JavaScript |
| Client | a **browser** (Infinite is the first) | browser |

## 2. Design principles

1. **Narrow waist.** Every browser implements this format or nothing interops.
2. **Renderer-agnostic.** Geometry is glTF + prefab references + a standard PBR
   material model — never a specific engine's internals. A second browser in a
   second language must be able to render a manifest with no Infinite code.
3. **Static-first, presence-optional.** A world is just files (manifest + glTF +
   WASM); host them on any static server, CDN, or IPFS. **Presence** (multi-user)
   is an *upgrade* enabled by naming a `presence.relay`. Without one, a world
   gracefully degrades to solo — exactly like a static web page.
4. **Meaning is native.** Any placement may carry a Codex slug; a browser
   "inspect" surfaces canonical lore.
5. **Time is an axis.** A world declares `environment.year`; the same place is
   addressable at another time via the Locator's `@when`.

## 3. The Locator (addressing)

```
thread://<host>/<path>[@<when>][#<place>]
```

- `host` — the world host authority (a domain).
- `path` — the world's path on that host (may be empty for the host root).
- `@when` — OPTIONAL timeline year to arrive at (first-class time navigation).
- `#place` — OPTIONAL named anchor (a `spawn.name` or `portal.id`) to arrive at.

Examples: `thread://archive.pixygon.io/codex-archive`,
`thread://market.pixygon.io/market#entry`, `thread://amebrak.pixygon.io/caul@0`.

## 4. Manifest structure

A manifest is a JSON document. Top-level fields:

| Field | Req | Meaning |
|-------|-----|---------|
| `thread` | ✔ | Format tag; MUST be `"thread/0.1"`. |
| `world` | ✔ | Metadata: `id`, `title`, `description`, `author`, `codex[]`, `license`. |
| `environment` | | `year` (the `@when`), `sky`, `bounds`. |
| `spawns[]` | | Arrival points; the first is the default, others are `#place` anchors. |
| `assets[]` | | External content by id: `{ id, uri, kind }`, `kind ∈ gltf|texture|wasm|audio|other`. |
| `prefabs[]` | | Deduplicated renderables keyed by `StructuredId`: `{ id, mesh, material }`. |
| `placements[]` | | Instances: `{ prefab, position, rotation, scale, codex?, behavior?, data }`. |
| `portals[]` | | Veils: `{ id, position, …, to (Locator), label, preview }`. |
| `behaviors[]` | | WASM modules: `{ id, wasm (asset id), on[] }`. |
| `presence` | | `{ relays[]?, relay?, max_occupants?, voice }`. Absent → solo. |

**Presence relays.** Prefer `relays` — an ordered list of interchangeable,
conformant relays (primary first, then fallbacks) — so no single relay URL is a
point of failure; a browser tries them in order and uses the first reachable one.
The singular `relay` is retained for backward compatibility. A world targeting both
old and new browsers MAY set `relay` (its primary) *and* `relays` (the full list);
the effective order is `relays` then `relay` if not already present. Any conformant
relay works — presence is federated, so a world names its *own* relay(s).

### Prefabs & materials

A `prefab.mesh` sets **exactly one** of `asset` (a glTF asset id) or `builtin`
(`cube`, `sphere`, `cylinder`, `capsule`, `plane`, `quad`). `material` is a
standard PBR block: `base_color_texture`/`orm_texture`/`normal_texture` (asset
ids) plus scalar `base_color`, `metallic`, `roughness`. Prefab ids use the
`StructuredId` `CCSSNNNN` scheme (imported/world prefabs live in category `60`).

### Portals

`to` is a destination **Locator** (MAY target another host — this is what makes
the Thread a *web* and not a walled world). `preview` ∈ `none | static | live`:
a `live` preview shows the real far side (crowd, weather, time-of-day) before you
step through. Crossing a portal to a new host performs an **identity handoff**
(your Passport, avatar, and inventory travel) — specified separately in the
Portal Handoff layer.

## 5. Conformance

- A **conformant manifest** parses as JSON, carries a recognized `thread/…` tag,
  and passes validation: every placement references a declared prefab; every
  prefab has exactly one mesh source that resolves; every behavior/asset
  reference resolves; every portal `to` is a valid Locator. The reference
  validator is `WorldManifest::validate`.
- A **conformant browser** MUST: resolve a Locator, fetch + validate a manifest,
  render prefabs/placements with the standard material model, and traverse
  portals. It SHOULD render `live` portal previews and join presence when a relay
  is present; it MAY ignore behaviors it cannot sandbox. Unknown fields MUST be
  ignored (forward compatibility).

## 6. Versioning

The `thread` tag is `thread/MAJOR.MINOR`. Minor versions are additive and
backward-compatible (browsers ignore unknown fields). Major versions may break.
The **conformance test suite** — not this prose — is the ultimate arbiter: the
two shipped worlds (`worlds/codex-archive`, `worlds/market`) are the canonical
fixtures and MUST validate under any implementation claiming `thread/0.1`.

## 7. Example

See [`worlds/codex-archive/world.json`](../../worlds/codex-archive/world.json)
(a walkable Codex viewer) and [`worlds/market/world.json`](../../worlds/market/world.json)
(a walkable market). They portal to each other across hosts — the Thread's "two
linked pages" moment.
