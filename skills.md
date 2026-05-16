# skills — signal-persona-auth

*Per-repo agent guide for the Persona ingress / provenance
vocabulary crate.*

---

## Checkpoint — read before editing

Before changing code in this repo, read:

- `~/primary/skills/contract-repo.md`
- `~/primary/skills/architecture-editor.md`
- `~/primary/skills/architectural-truth-tests.md`
- `~/primary/skills/nix-discipline.md`
- this repo's `ARCHITECTURE.md`.

---

## What this repo is for

`signal-persona-auth` defines typed provenance records for Persona
ingress: where a request entered the engine, which known route /
channel labels are attached, and which component principal it came
from. It is deliberately **not** an authentication library and does
not carry in-band proof material.

The crate is types-only — there is no `signal_channel!` declaration
because no relation crosses a Signal wire whose vocabulary is
provenance alone. Other contract crates import the typed records and
attach them to their own request/reply payloads.

---

## What this repo owns

- `EngineId`, `RouteId`, `ChannelId` — typed engine, route, and
  channel identifiers used by the daemon/router boundary.
- `ComponentName` — closed enum of supervised local Persona
  component principals.
- `ConnectionClass` — closed enum of known ingress classes.
- `OwnerIdentity` — engine ownership recorded from local system
  context.
- `MessageOrigin` — closed enum naming `Internal(ComponentName)` or
  `External(ConnectionClass)`.
- `IngressContext` — typed bundle of origin facts.

## What this repo does not own

- Authentication or authorization runtime.
- In-band signing or proof types.
- Component socket ownership or routing policy.
- Storage of any kind.
- Compatibility wrappers for legacy lock files.

---

## Load-bearing invariants

- **No in-band proof type.** Do not introduce `AuthProof` or any
  Persona-specific signing/credential record. The local trust
  boundary lives outside this crate (per-engine sockets,
  filesystem ownership, SO_PEERCRED). The `tests/round_trip.rs`
  source-scan witness asserts the absence of an `AuthProof`
  symbol.
- **Closed enums for known principals and provenance classes.**
  `ComponentName`, `ConnectionClass`, and `MessageOrigin` are
  closed. New variants land through coordinated schema bumps.
- **String newtypes only when the universe is outside this crate.**
  `EngineId`, `RouteId`, `ChannelId` are private-field newtypes
  with explicit text projections, not bare `String` fields.
- **Wire enums contain no `Unknown` variant.** No polling-shape
  placeholders.
- **No runtime code.** No Kameo, Tokio, socket, redb, terminal, or
  database logic in this crate.
- **Round trips cover every variant.** rkyv length-prefixed frame
  round trips in `tests/round_trip.rs`; canonical NOTA examples in
  `examples/canonical.nota` with a parser test.
- **SO_PEERCRED → ConnectionClass mapping is contract-stable.**
  The mapping documented in `ARCHITECTURE.md` §"SO_PEERCRED →
  ConnectionClass mapping" is not adjustable without a coordinated
  wire bump.
- **Pin upstream contracts via a named API reference.** Cargo deps
  declare `git = "..."` with a named branch/bookmark, never raw
  `rev = "..."`.

---

## Editing patterns

### Adding a new component principal

1. Add the variant to `ComponentName`.
2. Add the round-trip witness through rkyv and NOTA.
3. Update consumers that pattern-match on `ComponentName`.

### Adding a new connection class

1. Add the variant to `ConnectionClass`.
2. Document the SO_PEERCRED-uid → variant mapping in
   `ARCHITECTURE.md` §"SO_PEERCRED → ConnectionClass mapping".
3. Add the round-trip witness through rkyv and NOTA.
4. Update consumers' ingress dispatch.

---

## NOTA codec note

This crate has no `signal_channel!` declaration; there is no
payload-head-vs-variant-name distinction here. Records derive
`NotaRecord`, `NotaEnum`, or `NotaTransparent` directly; the text
head matches the type name.

---

## See also

- this workspace's `skills/contract-repo.md`.
- this workspace's `skills/architectural-truth-tests.md`.
- this workspace's `ESSENCE.md` §"Perfect specificity at
  boundaries" and §"Infrastructure mints identity, time, and
  sender" — the rules that shape this vocabulary.
- `signal-persona`'s `skills.md` and `signal-persona-message`'s
  `ARCHITECTURE.md` — consumers that import these provenance
  records.
