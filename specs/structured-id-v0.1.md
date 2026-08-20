# Structured Id — v0.1

**Status:** Draft · **Layer:** Naming (how a world addresses its own parts) ·
**Reference impl:** [`thread-structured-id`](https://crates.io/crates/thread-structured-id)

A **StructuredId** is the name a World Manifest gives to one of its parts. Every
`prefabs[].id` is one, and every `placements[].prefab` refers to one. It is the
only identifier the manifest requires and, until now, the only one this standard
described nowhere — it was defined by a crate, which is not the same as being
specified.

## 1. What it is

Eight decimal digits, in three fields:

```
CCSSNNNN
││││└┴┴┴─ NNNN  number       0001–9999   which one
││└┴───── SS    subcategory  00–99       a shelf within the kind
└┴─────── CC    category     01–99       the kind of thing
```

Its value is exactly

```
CC × 1 000 000  +  SS × 10 000  +  NNNN
```

so the digits and the number are the same fact written two ways. That is the
whole point: an id sorts numerically into its own categories, and a human
reading `60930001` can see it is a world-local prefab without a lookup table.

**Validity (normative).** An id is valid if and only if `01 ≤ CC ≤ 99` and
`0001 ≤ NNNN ≤ 9999`. `SS` may be anything in `00–99`, `00` meaning *general*.
Category `00` and number `0000` are reserved and MUST be rejected — which makes
`0` an invalid id rather than a plausible default, so a field left unset cannot
be mistaken for a real name.

## 2. On the wire (normative)

An id appears in JSON as **either** the canonical string **or** the equivalent
number:

```json
"60930001"     ← canonical
 60930001      ← equally valid, and the same id
```

- A **producer** MUST emit the canonical form: the value zero-padded to exactly
  eight digits, as a JSON string.
- A **consumer** MUST accept both forms and MUST treat them as equal. `"01000001"`
  and `1000001` are the same id; a consumer that distinguishes them is wrong.

Padding is why the string is canonical. Category 01 encodes to `1000001`, which
is seven digits as a number, and a consumer that pattern-matched on eight digits
would reject a legal id — so the string form always pads and the numeric form is
always compared by value, never by text.

A consumer MUST reject a string that is not all digits, and MUST reject any value
that fails §1. Rejecting means refusing *that reference*, not the world: the
containing manifest is invalid only where the spec's own conformance clauses say
so ([world-manifest-v0.1](world-manifest-v0.1.md) §5).

## 3. Categories

The category is what makes an id readable. These ranges are reserved by this
specification:

| CC | Domain | Contains |
|---|---|---|
| 01–09 | *(reserved)* | |
| 10–19 | Actors | player characters, people, creatures |
| 20–29 | Items | weapons, clothing, tools, materials |
| 30–39 | Skills | abilities, active and passive |
| 40–49 | World | doors, containers, signs, portals, and (46) worlds and hosts |
| 50–59 | *(reserved)* | |
| **60** | **World-local prefabs** | the renderables a single manifest declares |
| 61–99 | *(unassigned)* | available; see below |

**Category 60 is the one a world author needs.** A prefab declared by a manifest
is local to that manifest — it means nothing outside it — so it takes a 60-series
id and nobody has to coordinate with anyone to pick one.

**An unrecognised category is not an error (normative).** A consumer that meets
a category it has no meaning for MUST still treat the id as a valid *name*: it
can be stored, compared, and referred to. It simply carries no domain. This is
what lets the table above grow without invalidating worlds written before the
row existed — the same rule as unknown fields in a manifest, applied to names.

## 4. What an id is not

- **Not a URL.** It names a part *within* a world; addressing across worlds is
  [locator-and-resolution-v0.1](locator-and-resolution-v0.1.md).
- **Not globally unique.** Two worlds may both declare prefab `60000001`; they
  are different prefabs, and neither is wrong. Uniqueness is required only
  within one manifest.
- **Not a hash.** It is assigned, not derived. Content-addressed identity on the
  Thread is [Weft](weft-v0.1.md)'s, and is a different thing for a different
  purpose.

## 5. Conformance

| Clause | Severity | Passes when |
|---|---|---|
| ids are well-formed | Error | every `prefabs[].id` satisfies §1 |
| ids are canonical | Warn | every emitted id is the eight-digit string form of §2 — accepting the numeric form is required, producing it is untidy |
| references resolve | Error | every `placements[].prefab` names a declared prefab (world-manifest §5) |

## 6. Versioning

Additive: categories may be assigned, and a consumer must already tolerate ones
it doesn't know (§3). Narrowing the grammar, changing the encoding, or making
either wire form illegal is a new version.

## 7. Status

Draft, and describing something already load-bearing rather than proposing
something new: every world in the reference constellation is addressed this way,
and the [JSON Schema](../schema/world-0.1.schema.json) validates against this
document.
