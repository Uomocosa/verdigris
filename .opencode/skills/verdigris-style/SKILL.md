---
name: verdigris-style
description: Use before writing, editing or reviewing any Rust code or test in verdigris (rule engine, cards, MockGame tests). Repo-specific code-style choices of the user that the general lele-rs / lele-syntax-rs skills do not cover. Read together with lele-syntax-rs RATIONALE.md and SMELLS.md.
---

# verdigris-style

## 0. Load first

1. Load `lele-rs` (binds `lele-syntax-rs`) and `bevy-rs`.
2. Read `lele-syntax-rs/references/RATIONALE.md` and `references/SMELLS.md` **in full** —
   actually open the files, do not rely on the skill summary.
3. Read `AGENTS.md` "Verdigris rules" 1–10.

This file only adds what is specific to verdigris. When it conflicts with the linter, the
linter wins; when it conflicts with an AGENTS rule, stop and ask.

## 1. Model the game the way the Comprehensive Rules describe it

Correctness to the CR comes first, over shortcuts and over "simpler for now". When the CR says
where a behaviour comes from, the engine puts it there: if a rule grants an ability, the rule
system grants it (305.6 gives a basic land "{T}: Add {G}" even if its text box is empty), and
the card does not repeat it. Name things with the CR's own words (`ActivateManaAbility`, not
`TapForMana`; supertype `Basic`, not a `basic_land` constructor).

**Why:** the user believes a game described exactly as the rules describe it is easier to
implement and extend: each new rule slots into the place the CR already gives it, and cards
never have to fight the engine.

**How to apply:** before designing, find the CR rule that owns the behaviour and quote it. If
the simple design and the CR disagree, follow the CR, or stop and ask.

## 2. Named values over `Default`

When a value means "empty", "none" or "zero", name it with an associated constant and use the
constant, in code and in asserts. `Default::default()` hides the meaning.

```rust
// Good
assert_eq!(game.mana_pool(p1), cards::ManaPool::EMPTY);

// Bad
assert_eq!(game.mana_pool(p1), cards::ManaPool::default());
```

If a type needs `Default` (derives, builders), make `default()` return the named constant.

## Adding rules here

Add a section per decision the user makes about how verdigris code should look: a one-line
rule, a Good/Bad example, and (if it is not obvious) a **Why**. Keep general Rust taste in
`lele-syntax-rs` SMELLS.md instead.
