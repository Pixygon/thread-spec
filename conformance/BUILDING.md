# Building the conformance suite

The suite currently depends on the reference World Manifest implementation
(`infinite-manifest`) via a **git dependency** on the Infinite repo. That repo
is private today, so an external build needs access.

**Tracked follow-up (makes this fully standalone + public-buildable):** publish
the reference crate to crates.io as `thread-manifest` and change
`conformance/Cargo.toml` to `thread-manifest = "0.1"`. The suite's public API is
unchanged either way. See `GOVERNANCE.md`.

Once that lands, CI can run:

```bash
cargo run --manifest-path conformance/Cargo.toml -- worlds/   # exits 0 = CONFORMANT
cargo test --manifest-path conformance/Cargo.toml             # clause + corpus tests
```
