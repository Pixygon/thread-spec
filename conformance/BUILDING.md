# Building the conformance suite

Fully standalone — it depends on the reference World Manifest implementation from
**crates.io** (`thread-manifest`), imported under the local name `infinite-manifest`
via a package alias so the suite source is unchanged. No git or path dependencies;
anyone can clone and build it.

```bash
cargo run --manifest-path conformance/Cargo.toml -- worlds/   # exits 0 = CONFORMANT
cargo test --manifest-path conformance/Cargo.toml             # clause + corpus tests
```

Published crates: `thread-manifest` (the format) depends on `thread-structured-id`
(the `CCSSNNNN` id scheme). Both are versioned together; bump per GOVERNANCE.md.
