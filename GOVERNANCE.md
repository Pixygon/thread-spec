# Governance & versioning

The Thread is an open standard. This document says how it changes and why you can
trust that it won't be pulled out from under you.

## Principles

1. **The spec is the authority, not any implementation.** Where a browser and a
   spec disagree, the spec wins. Reference implementations (Infinite, Loom) are
   conveniences, not the standard.
2. **The conformance suite is the arbiter.** "Conformant" is defined operationally:
   your world or browser passes [the suite](specs/conformance-v0.1.md). Debates
   about intent are settled by adding a clause, not by fiat.
3. **Additive within a major version.** No silent breaking changes. Anything that
   would break an existing conformant world requires a new major version.
4. **Open by default.** Changes happen as public issues and pull requests against
   this repository. Anyone may propose; the process below decides.

## Version tags

Every World Manifest carries a format tag: `thread/<major>.<minor>`.

- **Minor bump (`0.1` → `0.2`)** — additive only. New optional fields, new clauses
  that only tighten previously-undefined behavior, new spec documents. An older
  browser MUST ignore fields it doesn't recognize; a newer browser MUST supply
  documented defaults for fields an older world omits. Existing conformant worlds
  stay conformant.
- **Major bump (`0.x` → `1.0`, `1.x` → `2.0`)** — reserved for changes that can
  break existing worlds or browsers. Requires a migration note and a deprecation
  window. Browsers SHOULD state which major versions they accept.

The conformance suite is versioned in lockstep: `conformance-v<major>.<minor>`
matches the spec set it checks. A world declares the version it targets; the suite
run against it uses that version's clauses.

## How a change lands

1. **Propose** — open an issue describing the problem and the smallest change that
   solves it. Prefer optional additions over required ones.
2. **Draft** — a PR that updates the affected spec doc(s) *and* adds or amends a
   conformance clause. A spec change without a matching conformance change is
   incomplete: if it can't be checked, it isn't in the standard.
3. **Prove** — show at least one implementation reading/writing the change, and the
   conformance suite passing (or newly failing on the intended case).
4. **Adopt** — merge bumps the minor version. Breaking proposals are batched toward
   the next major.

## Severity of clauses

Conformance clauses are **Error** (a real interop violation — fails the suite) or
**Warn** (a quality signal — reported, doesn't fail). Promoting a Warn to an Error
is a **breaking** change (some previously-passing worlds would fail) and follows
the major-bump rules. Adding a new Warn is additive.

## Stewardship

The standard is stewarded in the open in this repository. The steward's job is
procedural — triage issues, keep the specs and the suite in sync, cut versions —
not editorial control over who may build on the Thread. The permissive licenses
(CC-BY-4.0 for specs, MIT/Apache-2.0 for the suite) guarantee the standard can be
copied, implemented, and forked without permission. If stewardship ever fails the
community, the standard can be forked; that possibility is the ultimate check.
