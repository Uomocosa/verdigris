# verdigris-datalog-datafrog

Our own small Datalog engine: stratified semi-naive evaluation on top of
[datafrog](https://docs.rs/datafrog/2.0.1/datafrog/). Implements `verdigris_datalog_ir::traits::Engine`.

## Goal

Evaluate runtime-built verdigris rules quickly, deterministically and with no runtime dependencies, so that it can
run on every state change, on every peer, and later in wasm.

## How it works

1. `Program::stratify()` (from the IR) splits the rules into strata; negation only reads lower, completed strata.
2. Each rule is compiled once (`CompiledRule`, `WiredRule`) into a join plan over generic tuples
   `(key, Payload)`, where the key columns are the variables shared between body atoms.
3. Per stratum, a `datafrog::Iteration` runs until nothing changes:
   - positive atoms: `Variable::from_join`
   - negated atoms: `Variable::from_antijoin` against the completed lower strata
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

## Limits and known behaviour

- Comparisons between different `Value` variants follow the IR's derived `Ord` (`Entity < Sym < Int`).
- Relations are expected to keep one arity.
- No aggregation yet.
