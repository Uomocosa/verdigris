# verdigris-datalog-ir

Engine-agnostic Datalog intermediate representation: the shared language every Datalog backend in this
workspace speaks.

## Goal

Rules for verdigris legality are built **at runtime** from card text and the Comprehensive Rules, not written in
a macro at compile time. This crate is that runtime form: plain Rust data that any engine can evaluate, so the
same program and the same tests run on every backend.

## What is in it

| Item | Purpose |
|---|---|
| `Value` (`Entity`, `Sym`, `Int`) | A ground value. `Ord`, so all collections stay sorted. |
| `Term` (`Var`, `Val`), `Atom`, `Literal` (`Pos`, `Neg`, `Cmp`), `Rule`, `Rel`, `CmpOp` | The rule language: Horn clauses with negation and comparisons. |
| `Program` | A set of rules. `canonical()` sorts and dedups them; `stratify()` orders them into strata. |
| `Facts` | Ground facts per relation (`BTreeMap<Rel, BTreeSet<Vec<Value>>>`): `add`, `contains`, `matching`, `iter`. |
| `traits::Engine` | `fn evaluate(&self, &Program, &Facts) -> Result<Facts, EngineError>`. Every backend implements it. |
| `EngineError` | `Unstratifiable`, `Unsafe`, `Backend(String)`. |
| `Case` | The shared conformance suite: `Case::cases()` + `Case::check(&engine)`. |

## Important rules

- **Stratified negation only.** A cycle through a negated literal is rejected (`Unstratifiable`). Stratification
  gives one well-defined result per program, the same requirement GDL (General Game Playing) puts on game rules.
- **Safety** (GDL spec): every variable in a rule head, in a negated atom or in a comparison must also appear in a
  positive atom of the same body; otherwise `Unsafe`.
- **Deterministic.** Only sorted collections reach any output: same input, same result, on every peer.
- **Conformance is the contract.** A new backend is accepted when `Case::check(&MyEngine)` passes. The suite covers
  transitive closure, negation, comparisons, constants, independent strata, empty negated relations, entities
  (lossless round-trip), evaluating twice, and the two rejection errors. Add a case here, not inside one backend.

## Not in scope

Evaluation (see `verdigris-datalog-datafrog`) and anything MTG-specific (see
`verdigris-rule-datalog`).
