# frogmyr

Small stratified semi-naive Datalog engine on datafrog: the Datalog rule language as runtime
data, plus our engine evaluating it.

## Goal

frogmyr is a small, generic, deterministic stratified Datalog engine on datafrog: rules as
runtime data, no game knowledge. It supports one stratified `count` aggregation per rule.

Evaluate runtime-built rules quickly, deterministically and with no runtime dependencies, so that it can
run on every state change, on every peer, and later in wasm.

## What is in it

| Item | Purpose |
|---|---|
| `Value` (`Entity`, `Sym`, `Int`) | A ground value. `Ord`, so all collections stay sorted. |
| `Term` (`Var`, `Val`), `Atom`, `Literal` (`Pos`, `Neg`, `Cmp`, `Count`), `Rule`, `Rel`, `CmpOp` | The rule language: Horn clauses with negation, comparisons and stratified count. |
| `Program` | A set of rules. `canonical()` sorts and dedups them; `stratify()` orders them into strata; `evaluate(&facts)` runs them with the datafrog engine. |
| `Facts` | Ground facts per relation (`BTreeMap<Rel, BTreeSet<Vec<Value>>>`): `add`, `contains`, `matching`, `iter`. |
| `EngineError` | `Unstratifiable`, `Unsafe`, `Backend(String)`. |
| `Case` | The shared conformance suite: `Case::cases()` + `Case::check()`. |
| `CompiledRule`, `WiredRule` | One rule compiled into a join plan over generic `(key, Payload)` tuples. |
| `Payload` | The non-key columns carried through a join. |

## How it works

1. `Program::stratify()` splits the rules into strata; negation and `count` read only lower, completed strata.
2. Each rule is compiled once (`CompiledRule`, `WiredRule`) into a join plan over generic tuples
   `(key, Payload)`, where the key columns are the variables shared between body atoms.
3. Per stratum, a `datafrog::Iteration` runs until nothing changes:
   - positive atoms: `Variable::from_join`
   - negated atoms: `Variable::from_antijoin` against the completed lower strata
   - `count N: body`: distinct body bindings grouped by the head's group variables, joined through
     the synthetic `__frogmyr_count` relation
   - comparisons: `PrefixFilter`; comparisons between constants are folded at compile time
4. The completed relations are absorbed into the output `Facts`.

## Why datafrog

- It provides the semi-naive delta bookkeeping (`recent` / `stable` / `changed()`), the part hand-written engines most
  often get wrong.
- Its relations are sorted `Vec`s, so iteration order is deterministic by construction.
- Pure Rust, no dependencies, no threads, no `unsafe` in our code.
- Last release 2.0.1 is from 2019, but the repository is still maintained and it underpins the Polonius
  borrow-checker work.

## Numbers (debug build, from the D1 and D3 reports)

- Shared conformance suite: under 1 ms.
- A 300-node chain (44,850 derived tuples): about 640 ms. That is far larger than a game state.
- Play legality on the r305 test states: about 10.5 ms in total for 16 candidates.

## Important rules

- **Stratified negation and count.** A cycle through a negated literal or a `Count` literal is rejected
  (`Unstratifiable`); a count reads only completed lower strata. Stratification gives one well-defined result per
  program, the same requirement GDL (General Game Playing) puts on game rules.
- **Safety** (GDL spec): every variable in a rule head, in a negated atom or in a comparison must also appear in a
  positive atom of the same body, or be bound by a `Count` literal (its output variable and the head variables it
  groups by); otherwise `Unsafe`.
- **One count per rule.** A second `Literal::Count` in the same body is rejected as `Unsafe`, naming that count's
  output variable.
- **Deterministic.** Only sorted collections reach any output: same input, same result, on every peer.
- **Conformance is the contract.** `Case::check()` passes. The suite covers
  transitive closure, negation, comparisons, constants, independent strata, empty negated relations, entities
  (lossless round-trip), count grouping, evaluating twice, and the three rejection errors. Add a case here, not
  inside one backend.

## Limits and known behaviour

- Comparisons between different `Value` variants follow the IR's derived `Ord` (`Entity < Sym < Int`).
- Relations are expected to keep one arity.
- A `count` body holds positive atoms only: no negation, comparison or nested count.
- Groups with no body rows produce no row: zero counts are not derived.
- `__frogmyr_count` is an engine-internal relation; a user relation of that name is shadowed in a rule with a count.

## Not in scope

Anything MTG-specific (see `verdigris_rule_engine::legality`).
