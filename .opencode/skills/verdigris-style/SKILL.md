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

## 3. MockGame tests: setup → look up → act → assert

The verdigris form of SMELLS S10. Right after `MockGame::new(...)`, look up every card the
test uses with `exactly::<N>(CARD)?`; then write the actions as one `game.play([...])` script
built from the verb helpers (`cast`, `targeting`, `activate_ability`, `declare_attackers`,
`declare_blockers`, `play_land`, `discard`, `pass_to`); then assert. Tests return
`Result<(), TestError>` and use `?` (keep the turbofish explicit: `exactly::<1>`).

```rust
// Good
let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
game.play([p1.pass_to(DECLARE_ATTACKERS).declare_attackers([bears]), p2.pass_to(DECLARE_ATTACKERS)])?;
assert!(game.app.world().get::<cards::Attacking>(bears.into()).is_some());
Ok(())

// Bad — play first, look up after, raw Action built by hand
game.play([p1.pass_to(DECLARE_ATTACKERS), p2.pass_to(DECLARE_ATTACKERS)]).unwrap();
let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
game.act(p1, actions::Action::DeclareAttackers(vec![bears.into()]));
```

Raw `game.act(..)` stays only in tests that check the engine rejects an action
(`game.last_rejection()`). A card that only exists after an action (drawn, created) is looked
up right after that step, with a comment.

**Why:** the reader learns what is on the table before reading what happens to it, and a
lookup placed after `play` silently depends on the card not having moved.

## Adding rules here

Add a section per decision the user makes about how verdigris code should look: a one-line
rule, a Good/Bad example, and (if it is not obvious) a **Why**. Keep general Rust taste in
`lele-syntax-rs` SMELLS.md instead.
