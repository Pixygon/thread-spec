# Locator & Resolution — v0.1

**Status:** Draft · **Layer:** Addressing (the URL + DNS of the Thread) · **Reference impl:** `src/resolver.rs`

How a `thread://` address is written, and how a browser turns it into a world —
**including the decentralized path that lets anyone host a world on their own
domain with zero registry and zero contact with Pixygon.**

## 1. The Locator (grammar)

```
thread://<host>/<path>[@<when>][#<place>]
```
- `host` — the world host authority (a domain).
- `path` — the world's path on that host (may be empty → the host's root world).
- `@when` — OPTIONAL timeline year to arrive at.
- `#place` — OPTIONAL named anchor (a `spawn.name` or `portal.id`).

`@when` and `#place` are **client-side** navigation *within* a resolved world; they
never affect which document is fetched.

## 2. Resolution algorithm (normative)

Given a Locator, a conformant browser resolves it in this order, using the first
that yields a valid manifest:

1. **Registry / resolver** (OPTIONAL, a value-add). If the browser is configured
   with a resolver, `GET <resolver>/thread/resolve?loc=<url-encoded locator>` →
   `{ world, manifestUrl, assetBase, presenceRelay?, codexBase?, passportIssuer? }`;
   then `GET manifestUrl`. Registries provide discovery, presence, and identity —
   but a browser MUST NOT *require* one.
2. **`.well-known` (decentralized, no registry).** Fetch the host's own manifest
   directly (§3). This is the path that makes the Thread an open web: it needs
   nothing but static hosting on the author's domain.
3. **Local** (dev/offline) — an implementation MAY resolve to a local file.

A browser MUST implement at least step 2. Steps 1 and 3 are optional conveniences.

## 3. The `.well-known/thread` convention (self-hosting)

To publish a world on `example.com` with **no registry**, serve a manifest at:

```
https://<host>/.well-known/thread[/<path>]/world.json
```
- `thread://example.com`            → `https://example.com/.well-known/thread/world.json`
- `thread://example.com/gallery`    → `https://example.com/.well-known/thread/gallery/world.json`

**Host requirements (normative):**
- Serve the manifest over **HTTPS**.
- `Content-Type: application/json`.
- **`Access-Control-Allow-Origin: *`** — browsers fetch cross-origin; without CORS
  the world won't load.
- The body MUST be a conformant `thread/0.x` World Manifest (validates against the
  reference validator).
- Relative asset URIs in the manifest resolve against the manifest's directory
  (`https://<host>/.well-known/thread[/<path>]/`). Absolute / `ipfs://` URIs pass
  through. Assets SHOULD also be served with permissive CORS.
- Portals to other hosts just carry those hosts' Locators — no coordination needed.

That's the whole contract. A `world.json` + assets on any static host = a place on
the Thread, reachable by any browser, forever, with zero contact with anyone.

## 4. Registry (optional value-add)

A registry (e.g. Pixygon's `GET /v1/thread/resolve`) MAY:
- resolve **its own** registered worlds (canonical `thread://` names, minted ids),
- **fall back** to a host's `.well-known` for unregistered hosts (server-side),
- attach discovery/presence/identity endpoints to the response.

None of this is required to author, host, or visit a world. The registry is Chrome's
Google-account layer, not the web.

## 5. Conformance

- A **conformant host** serves a valid manifest at the `.well-known` path with
  HTTPS + JSON + CORS `*`.
- A **conformant browser** resolves a bare `thread://host[/path]` via `.well-known`
  with no registry configured, applies `@when`/`#place` client-side, and renders
  the manifest. `thread doctor <host>` (tooling) checks a host end to end.
