# Contributing to the Thread

Anyone may propose a change. The steward's job here is procedural — triage,
keep the specs and the suite in sync, cut versions — not deciding who is
allowed to build. If you are reading this because something in the standard is
wrong, unclear, or missing, you are the person this document is for.

## Before you open anything

**Run the suite.** It settles more questions than a discussion does, and you
need none of our code to do it — it installs from crates.io:

```bash
cargo install thread-conformance

thread-conformance worlds/                        # a corpus of <name>/world.json
thread-conformance --live yourdomain.com          # a live host, over .well-known
thread-conformance --relay wss://your-relay       # a presence relay
thread-conformance --rendezvous wss://your-rv     # a P2P rendezvous
```

If the suite disagrees with a browser, **the suite and the spec are right and
the browser is wrong** — including when the browser is ours. Say so in an issue.

## The four steps

1. **Propose.** Open an issue describing the problem and the smallest change
   that solves it. Prefer optional additions over required ones: every required
   field is a tax on every implementer, forever.
2. **Draft.** A pull request that updates the affected spec **and** adds or
   amends a conformance clause. This is the load-bearing rule — *a spec change
   without a matching conformance change is incomplete.* If it cannot be
   checked, it is not in the standard.
3. **Prove.** Show at least one implementation reading or writing the change,
   and the suite passing — or newly failing on exactly the case you intended.
4. **Adopt.** Merging bumps the minor version. Anything that would break an
   existing conformant world is batched toward a major version.

Full rules, including clause severity and the version contract:
[GOVERNANCE.md](GOVERNANCE.md).

## What makes a good proposal

- **Name the world it breaks.** A change justified by a concrete world, host or
  browser that cannot express something today is far stronger than one
  justified by symmetry or taste.
- **Say what a browser must do with it.** A field nobody is obliged to read is
  documentation, not a standard.
- **Say what happens to a world that omits it.** Every optional field needs a
  documented default, or older worlds silently change meaning.
- **Prefer Warn over Error** for anything that is a quality signal rather than
  an interop violation. A clause that fires on correct worlds teaches authors
  to ignore the report — which costs more than the clause was worth.
- **Preserve what you don't understand.** A reader may ignore unknown fields; a
  writer may not drop them. A tool that reads a world, changes something and
  writes it back MUST carry unrecognised fields through unchanged.

## Things we would particularly like

These are the thin places, named honestly rather than as a wishlist:

- **A second browser.** One implementation is an implementation; two is a
  standard. The suite exists precisely so a second one can prove itself without
  asking us.
- **A second relay.** The presence wire is public and the reference relay is
  small. Worlds naming their own relay is what stops presence becoming a centre.
- **A second Passport issuer.** Identity is designed keys-only and federated,
  and has only ever had one issuer. Another would prove the seam.
- **Worlds.** Portals are the links that make this a web. A world nobody links
  to is a page nobody links to.
- **Prose for the StructuredId scheme**, which is normatively required by the
  manifest and currently defined only by a Rust crate.

## Licensing

Specs are CC-BY-4.0; the conformance suite is MIT/Apache-2.0. By contributing
you agree your contribution is licensed the same way. Those licences are
deliberate: the standard can be forked away from its steward, which is the only
real guarantee that it stays open.

## Conduct

Be straightforward and assume competence. Argue about the artefact, not the
person. If you think something here is wrong, the useful thing is a case that
demonstrates it — bring the world that breaks, and the argument makes itself.
