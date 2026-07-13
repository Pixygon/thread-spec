# Presence Wire — v0.1

**Status:** Draft · **Layer:** Presence · **Owner:** main agent (relay), co-designed with Infinite (client interpolation)

The wire format for shared presence on the Thread. Transport: **WebSocket** first
(`wss://<relay>/thread/:worldId`), path to WebTransport/QUIC later. This spec fixes
the coordinate frame, the pose message, and the timing model so the relay and any
browser interpolate identically.

## 1. Coordinate frame (normative)

Match the engine + glTF: **right-handed, +Y up, meters**. Rotation is a unit
**quaternion `[x, y, z, w]`**. This is the same frame the World Manifest uses, so
poses need no conversion.

## 2. Session

- `join` (client→relay): `{ t:"join", passport, spawn?: string }` — relay verifies
  the Passport against the issuer `jwks.json`, assigns an occupant `id` (u32), and
  replies `welcome { id, occupants:[Occupant], tick_hz }`.
- `leave` (either way): `{ t:"leave", id }`.
- The relay is authoritative on the occupant list and does area-of-interest culling
  (only relay poses within a radius / same sub-area).

## 3. Pose (the hot path)

Clients send at **10–20 Hz**; the relay fans out (optionally batched). Wire fields:

```
{ t:"pose", id:u32, ts:u32,          // ts = relay-epoch milliseconds (u32 wraps ~49d; fine)
  p:[f32;3],                          // position (m)
  r:[f32;4],                          // rotation quaternion xyzw (MAY send yaw-only via `y:f32` for bandwidth)
  v:[f32;3],                          // linear velocity (m/s) — enables extrapolation
  a:u8 }                              // animation/locomotion state enum (idle/walk/run/jump/…)
```

Bandwidth profile (v0.1 keeps it simple; quantization is an optional relay
capability, not required for conformance):
- **Full:** floats as above (~40 B/pose).
- **Quantized (optional):** `p` as int16 cm relative to an area origin; `r` as
  smallest-three (3×int16 + 2-bit index); `v` as int8 dm/s. Negotiated in `welcome`.

## 4. Timing & interpolation (client contract)

- The relay stamps every pose with `ts` on receipt (single clock source; clients
  never trust each other's clocks).
- Clients render presence **~100 ms in the past** (interpolation buffer), lerping
  position and **slerping** rotation between the two poses that bracket
  `render_time = latest_ts - 100ms`.
- On a gap (missing pose), **extrapolate** with `v` for up to ~250 ms, then freeze.
- The local player is never interpolated (rendered from live input).

This is why `v` and a server `ts` are mandatory: they're exactly what the client
needs to hide jitter without trusting peer clocks.

## 5. Interactions & voice

- `interact` (client→relay→others): `{ t:"interact", id, target, action, data }` —
  mirrors the Behavior ABI Action `presence.emit`, so a world's WASM can broadcast
  a gameplay event (a lever pulled, an item shown) to co-present travelers.
- `voice` (optional): `{ t:"voice", id, seq, opus }` — Opus frames, spatialized by
  the receiver using the sender's last `p`. Absent if `presence.voice=false`.

## 6. Conformance

- A **conformant relay** verifies Passports on `join`, stamps `ts`, maintains the
  occupant list, and fans out `pose`/`interact` within area-of-interest.
- A **conformant browser** buffers ~100 ms, interpolates per §4, and renders other
  travelers' avatars from their Passport descriptor (`GET /v1/passport/avatar/:sub`).
- Presence is **optional** (per the manifest): no `presence.relay` → solo, no wire
  traffic at all.
