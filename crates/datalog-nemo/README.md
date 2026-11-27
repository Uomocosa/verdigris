# verdigris-datalog-nemo

A second backend for the same `verdigris_datalog_ir::traits::Engine`, backed by
[Nemo](https://github.com/knowsys/nemo), a full Datalog rule engine from TU Dresden.

## Goal

Compare our own engine against a mature, full-featured Datalog engine on exactly the same programs and tests. Nemo also
offers stratified negation, aggregates and existential rules, which we may want later.

## How it works

1. The IR's `stratify` runs first, so rejection behaviour matches the datafrog engine exactly.
2. `render`: the IR `Program` plus the input `Facts` become Nemo program text: `~` for negation, infix comparisons,
   folded constant comparisons, and an `@export` for every derived relation so it can be read back.
3. `nemo::api::load_string` and `nemo::api::reason` run on a **single-threaded** tokio runtime
   (`Builder::new_current_thread()`). Never multi-threaded, to stay deterministic.
4. The rows of each predicate are read back and decoded into `Facts`.

## Encodings

- `Value::Entity(n)` is rendered as the IRI `<https://verdigris.invalid/entity/n>` and decoded back exactly. Entities
  never come back as `Int`.
- `Value::Sym` is read back through a table built from the symbols in the program and input facts, once per
  `evaluate`. An unknown symbol is an `EngineError::Backend`. No leaked memory, checked by evaluating 1,000 times in a test.

## Dependency

- `nemo` is **not on crates.io**; it is a git dependency pinned to
  `7706c48f810249879cee1406b2c5882921584323` (main, 2026-11-04). Nemo describes itself as "in heavy development".
- About 513 lines of transitive dependencies (`cargo tree`), and a first build of about 2 min 17 s.

## Numbers (debug build, from the D2 and D3 reports)

- Shared conformance suite: about 15 ms (datafrog: under 1 ms).
- Play legality on the r305 test states: about 202 ms in total, mostly the per-call runtime set-up and the text round-trip.

## Limits

Every `evaluate` parses a fresh program, so it costs more per call than an engine that keeps state. Wasm support exists
upstream (`nemo-wasm`) but is not tried here.
