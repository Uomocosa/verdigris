# verdigris-lint

**Goal: one linter for verdigris — every `lele-lint` rule plus every `lele-bevy-lint` rule, with code-defined `PathExemption`s for the `__rules__/` convention (one Bevy system per MTG Comprehensive Rule, named after the rule, with the CR text above it).**

## What it does

`verdigris-lint` is a binary that wraps `lele_lint::Linter`:

- all `LELE0xx` rules from `lele-lint`;
- all `LELEBEVY0xx` rules from `lele-bevy-lint` (`BevyLinter::bevy_rules()`);
- a code-defined list of `PathExemption`s in `src/constants.rs` for paths under `__rules__/`.

`__rules__/` is a sibling of `src/` (only `crates/rule-engine` has one). The binary walks it as an extra source root and prefixes its relative paths with `__rules__/`, so every rule sees those files. It is not added to `[lele.layout] roots`, because that switches lele-lint to multi-root mode and forces every `__methods__` body into per-root subfolders.

## Usage

```bash
cargo run --quiet -p verdigris-lint -- --config lele.toml crates/rule-engine
cargo run --quiet -p verdigris-lint -- --rule-list
cargo run --quiet -p verdigris-lint -- --only LELE010 crates/rule-engine
cargo run --quiet -p verdigris-lint -- --sync-methods --config lele.toml crates/rule-engine
```

`just lint` runs it once per crate. `just check-crate <crate>` runs clippy and then this linter.

## Exemptions

Each `PathExemption` carries a real reason in `src/constants.rs`. A new exemption needs a reason explaining why the `__rules__/` convention conflicts with the rule; adding one is the user's decision.
