# Presence Topology — v0.1

**Status:** Draft · **Layer:** Presence · **Companion to:**
[presence-wire-v0.1](presence-wire-v0.1.md)

The wire spec fixes *how two endpoints exchange poses*. This spec fixes *how
presence is organized* so it is **scalable, seamless, and survivable** — able to
hold large crowds, hide jitter, and keep working when any single operator (Pixygon
included) disappears. The governing principle mirrors decentralized hosting:

> **Presence is federated. A world names its own relay(s); no global server is
> required, and no single party owns the network.** The standard is the wire, not
> any server.

## 1. Why not one relay

A single global relay is the presence equivalent of a single registry: a bottleneck,
a scaling ceiling, and a single point of death. It fails all three requirements —
so the topology is designed to avoid it at every level.

## 2. The tiers

A world picks a presence tier the way it picks hosting: **static-first, upgrade as
needed.** Each tier is independent of any specific operator.

| Tier | Who runs it | Scale | Survives operator loss |
|------|-------------|-------|------------------------|
| **0 · Solo** | nobody | 1 | trivially (no traffic) |
| **1 · P2P mesh** | the participants | small (≤ ~16) | fully — no server at all |
| **2 · Relay** | any host (the world's own) | thousands / instance | yes — relay is commodity, self-hostable, swappable |
| **3 · Sharded / federated** | many relay instances | very large | yes — no instance is special |

A world declares its tier implicitly through its manifest `presence` block:

```jsonc
"presence": {
  "mode": "relay",                       // "solo" | "p2p" | "relay" (default inferred)
  "relays": ["wss://a.example/thread/x",  // ordered: primary, then fallbacks
             "wss://b.example/thread/x"],
  "relay": "wss://a.example/thread/x",    // legacy single (compat; see relay_list())
  "max_occupants": 64,
  "voice": true
}
```

Absent a relay/mode → **Tier 0**. This is the whole of the required surface; the
rest of this document is how browsers and relays behave within it.

## 3. Tier 1 — P2P mesh (serverless)

For small gatherings, presence needs **no dedicated server**. Peers connect
directly (WebRTC data channels), each sending its pose to the others. The only
shared infrastructure is a **stateless rendezvous** used once, at join, to
introduce peers (exchange SDP/ICE) — it never sees poses and holds no session
state, so it can be a tiny edge function, a public signaling service, or even
another peer.

- **Coordinate frame, pose shape, timing** — identical to the wire spec. A peer is
  just a relay-of-one from its neighbours' point of view.
- **`ts` in a mesh:** with no server clock, the *sender* stamps `ts` from its own
  monotonic clock and peers interpolate against *that sender's* recent timeline
  (never across senders). Velocity `v` still drives extrapolation.
- **Bound:** meshes are O(n²) in connections; cap at ~16 occupants, then a world
  SHOULD upgrade to Tier 2. Discovery of the cap is via the rendezvous.
- **Survivability:** if every server on earth is gone, two people who can reach
  each other still share presence. This is the floor the whole design guarantees.

## 4. Tier 2 — Relay

One conformant relay instance per world (the reference is
[`thread-relay`](../../crates/thread-relay)). The relay assigns occupant ids, stamps
`ts`, maintains the occupant list, and fans out within **area-of-interest** (§6).

- **Federation:** the world names the relay; different worlds (even different hosts)
  use different relays. There is no cross-world coupling and no central directory.
- **Failover:** `relays` is an ordered list. A browser tries them in order and uses
  the first reachable one; if its relay dies mid-session it MAY reconnect to the
  next (re-`join`, receive a fresh `welcome`). No single URL is a point of failure.
- **Statelessness:** a relay holds only ephemeral occupant positions. A restart just
  means clients reconnect — nothing durable is lost, so instances are cattle, not
  pets, and horizontal scaling is free.

## 5. Tier 3 — Sharding & relay federation

A single very large world scales by **sharding into cells** (by area, or by named
sub-zone). Each cell is an independent relay responsibility; a client is in exactly
one cell at a time and hands off at boundaries (re-`join` the neighbouring cell,
which the current relay advertises).

For a crowd spanning cells handled by *different relay instances*, relays **federate
at the boundary**: neighbouring relays exchange only the poses of occupants near a
shared edge (a coarse, low-rate summary), so a traveler sees the crowd on the far
side of a boundary without every relay holding every occupant. Full-mesh relay
federation is explicitly **out of scope for v0.1** — Tier 2 + area-of-interest
already scales to thousands per instance, which covers essentially all worlds. This
section fixes the *direction* so the wire and manifest don't foreclose it.

## 6. Scale mechanisms (normative for Tier 2+)

- **Area-of-interest (AoI):** a relay fans a pose only to occupants within a radius
  (or same sub-cell) of the sender. This is the primary scale lever — occupancy can
  grow far beyond what any client renders because each client only hears its
  neighbourhood. AoI radius is a relay/world parameter, not wire-visible.
- **Send-rate & interpolation:** clients send 10–20 Hz; receivers render ~100 ms in
  the past and interpolate (per the wire spec), so perceived smoothness is
  decoupled from packet rate and loss.
- **Optional quantization:** the wire spec's quantized profile (int16 positions,
  smallest-three rotations) is a relay capability negotiated in `welcome`; it cuts
  bandwidth per pose without changing the topology.

## 7. Discovery & identity

- **Relay discovery** is *in the world* — the manifest lists the relays. There is no
  global relay registry to capture or lose. A directory world MAY aggregate links,
  but it is never required for two people to be present together.
- **Identity** is the Passport (portable, provider-agnostic). A conformant relay
  verifies the Passport on `join` against the issuer's `jwks.json`; a self-hosted or
  dev relay MAY run open. Presence never requires a *Pixygon* account — any issuer a
  browser trusts works.

## 8. Survivability checklist (the "if the operator vanishes" test)

- ✅ No global server: worlds name their own relays; Tier 1 needs none.
- ✅ Relays are open-source & self-hostable: the reference relay runs on any box.
- ✅ Relays are swappable: `relays[]` fallback + open wire → repoint at will.
- ✅ Relays are stateless: nothing to back up or lose.
- ✅ Specs + reference code are permissively licensed and forkable.

## 9. Conformance

- A **Tier 2 relay** is conformant per [presence-wire-v0.1 §6](presence-wire-v0.1.md)
  and MUST honour `ts` stamping, the occupant list, and AoI fan-out. Certify it with
  `thread-conformance --relay wss://…`.
- A **browser** MUST support Tier 0 and SHOULD try `relays` in order (Tier 2
  failover). P2P (Tier 1) and sharding (Tier 3) are OPTIONAL capabilities a browser
  advertises; a browser that supports only Tier 2 still interoperates with every
  relay-backed world.
- A world is conformant at whatever tier it declares; declaring none is Tier 0 and
  always valid.

## 10. Status

v0.1 **specifies** all four tiers and **implements** Tier 0 (solo), Tier 2 (relay +
`relays[]` failover + AoI), and the **Tier 1 mesh core** (`loom::mesh` —
`MeshCoordinator` + the `MeshTransport` seam + the [`Signal`] rendezvous wire;
pose fan-out and peer management verified two-peer with an in-memory transport) and
the **WebRTC transport binding** (`loom::mesh_webrtc`, feature `p2p` — real data
channels + rendezvous SDP/ICE signaling + perfect-negotiation; compiles, feature-
gated). The remaining Tier-1 piece is deploying a **stateless rendezvous service**
to introduce peers (and live two-peer verification). Tier 3 (sharding/federation) is
specified and not yet implemented; the
manifest `mode`/`relays` surface and the per-sender `ts` semantics are designed so
both remaining pieces land without a breaking change.

[`Signal`]: ../../crates/loom/src/mesh.rs
