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
4. For rule systems, game actions, card effects and rule-changing cards: `verdigris-rules-style`.

This file only adds what is specific to verdigris. When it conflicts with the linter, the
linter wins; when it conflicts with an AGENTS rule, stop and ask.

## 1. Model the game the way the Comprehensive Rules describe it

Correctness to the CR comes first, over shortcuts and over "simpler for now". When the CR says
where a behaviour comes from, the engine puts it there: if a rule grants an ability, the rule
grants it, through the layers (§5) (305.6 gives a basic land "{T}: Add {G}" even if its text box is empty), and
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
`TestResult` and use `?` on every lookup and `play`, also in tests that expect an error
(only the expected failure keeps `unwrap_err()`; keep the turbofish explicit: `exactly::<1>`).

```rust
// Good
let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
game.play([p1.pass_to(DECLARE_ATTACKERS), p2.pass_to(DECLARE_ATTACKERS), p1.declare_attackers([bears])])?;
assert!(game.app.world().get::<cards::Attacking>(bears.into()).is_some());
Ok(())

// Bad — play first, look up after, raw Action built by hand
game.play([p1.pass_to(DECLARE_ATTACKERS), p2.pass_to(DECLARE_ATTACKERS)]).unwrap();
let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
game.act(p1, actions::Action::DeclareAttackers(vec![bears.into()]));
```

`play` starts the `pass_to` auto-passes only once every player's `pass_to` is listed, so an
action that must happen IN a step goes after all of them as its own entry, never chained onto
`pass_to` (`p1.pass_to(X).declare_attackers(..)` declares before step X starts). Chains of
actions with no `pass_to` (`activate_ability(..).cast(..).targeting(..)`) are fine.

Raw `game.act(..)` stays only in tests that check the engine rejects an action
(`game.last_rejection()`). A legal action has a verb (`p1.concede()`); if one is missing, add it in
`src/testing/__app__/` instead of using `act`. No raw `game.app.update()` either: a battlefield static applies
after the first `pass_priority` step, a full pass rotation also ends the step (pools empty), and
`p.pass_to(STEP)` reaches a later step. Answering a hook (`ChooseMay`, 103.6) is a legal action too: it
has no verb yet, so the next test that needs it adds `choose_may` there. Only a card that did not exist at setup (a token) is looked up
later, right after the step that creates it; an end-state check uses `.has(card)`.

**Why:** the reader learns what is on the table before reading what happens to it, and a
lookup placed after `play` silently depends on the card not having moved.

## 4. Test levels and fixtures

Each scenario is tested at exactly one level (RATIONALE §21, SMELLS S12):

| Level | Where | What it tests | Style |
|---|---|---|---|
| Method | `__methods__/…` `test_usage` | how to call it, one minimal case; honest corners as `#[rstest]` | `testing` fixtures |
| Rule | `__rules__/` with `MockGame` (tests named for the behaviour they check, no `test_usage`) | simple tests, one rule at a time, the smallest setup that shows it | `MockGame` + fixtures |
| Scenario | `tests/` (planned, does not exist yet) | complex games where several rules interact, an expected outcome, detailed asserts | `MockGame` |

A method test does not repeat what its rule test shows; a `tests/` scenario exists only
when several rules interact.

**Smell — rule test named after the rule id.** The file name already carries the rule ids;
the test name says the behaviour.

```rust
// Bad
fn r702_19b_trample_assigns_excess_damage_to_the_defending_player()

// Good
fn trample_assigns_excess_damage_to_the_defending_player()
```

Cards in tests come from `testing` fixtures (`testing::GARRUKS_COMPANION`,
`testing::GRIZZLY_BEARS`, …; call `(FIXTURE.definition)()` where a `CardDefinition` is
needed), never `cards::creature(..)` built in a test (RATIONALE §20, SMELLS S11). A
missing card gets a new fixture in `crates/rule-engine/src/testing/__fixtures__/`.
A new fixture is a real card whenever the engine can express its whole printed text (Ranger's Guile, not
`TEST_HEXPROOF_GRANT`). `TEST_*` cards are placeholders until enough rules exist for a real card; never add one
when a real card fits.
A static ability of a permanent (`statics::all(..)`, `TEST_NO_BURIALS`) only reaches the battlefield; to
change an event for a card on the stack or in a library, give that card its own replacement
(`TEST_NEVER_BURIED`).

```rust
// Good
let attacker = (testing::GARRUKS_COMPANION.definition)();

// Bad — hand-built card in a test
let attacker = cards::creature("Garruk's Companion", cards::Cost::default(), pt!(3, 2)).keyword(cards::trample());
```

The crate's own tests use `crate::testing`: `crates/rule-engine/Cargo.toml` enables the
`testing` feature for the dev-dependency on itself (lines ~18-21), and `src/lib.rs`
gates `pub mod testing` on that feature (line ~11).

**Why:** one copy of each card and each scenario, so a rules change touches one place.

## 5. Derived characteristics (CR 613)

Game objects have SOURCE components (written by actions/effects: `CardDefinition`, `InZone`,
`Owner`, `Controller`, `Damage`, `Tapped`, `Counters`, `ContinuousEffect` entities) and
DERIVED components (`EffectivePT`, type markers like `Creature`, `OnBattlefield`,
`StaticAbilities`, `TriggeredAbilities`). Derived components are written
only by the `r613_1_layers` system, `OnBattlefield` included. Rules filter on
them with `With<>`/`Without<>` and never re-derive them. Keywords live only in
characteristics: a condition asks `rules::has_keyword(..)`, a rule body asks
`Rules::characteristics(..)`.

Static abilities apply live (CR 611.3a); effects from a resolving spell are
`ContinuousEffect` entities whose affected set is fixed at resolution (CR 611.2c).

One game event = one message stage; layers recompute after each event and inside the
117.5 priority loop. A Bevy frame means nothing in game terms.

Invariant: removing all derived components and re-running layers yields the same world.

Observers/hooks only maintain structural invariants, never CR rules (triggers, SBAs,
replacement effects), because observers fire immediately and break the simultaneity of
704.3 / 510.2 and the "triggers wait for priority" timing of 603.3.

**Why:** Bevy does the filtering, and a rule stays correct when a new card grants or
removes an ability (e.g. Jump: "Target creature gains flying until end of turn").

**How to apply:** when a rule needs a characteristic (power/toughness, types, keywords,
whether something is on the battlefield), read the derived component in the query filter,
never the source component in the body. Type markers exist only for objects on the
battlefield: a rule about a spell on the stack or a card elsewhere still reads `CardDefinition`.
A keyword that changes a rule needs no marker and is never named in a rule body: it is a rule change in
its keyword file's `abilities` (one file per keyword in `__keywords__/`), and `Rules::mods` reads current characteristics, so a granted keyword
works too. Pick the change by what the CR sentence does: "can't" vetoes an event (`Restrict`: indestructible
→ `Destroy`, hexproof → `BecomeTarget`); "may … anyway" waives a restriction (`Allow`/`AsThough`: haste, trample →
510.1c `ONLY_BLOCKERS`); "is considered ..." widens a definition (`Allow` on a `Definition`: deathtouch → `LETHAL_DAMAGE`). The rule asks
`answers.legal(..)` / `answers.defines(..)` / `answers.value::<V>(entity)`, a keyword question
travels as a `rules::has_keyword(..)` condition, never a `has_x: bool` parameter. Switching a
rule from a keyword read to a rule change fixes granted keywords: red first, with a real card that grants it.
If a rule about the battlefield needs a marker that doesn't exist yet, add it to the layers in its
own refactor commit, before the rule reads it: the struct in `cards/__basic__/components.rs` (+ re-export), a
`Has<..>` field in `CurrentMarkers`, a `sync_marker` call in `write_markers`, and its line in that file's
identity test. If reading the marker fixes a bug,
switching the rule to it is the red-first fix commit (`r510_4`: markers, then first strike granted by an effect).

A rule that grants an ability (§1, 305.6) supplies the data; the layers write the marker. No `__rules__`
system inserts a derived marker itself.

```rust
// Good
attackers: Query<(Entity, &cards::Controller), With<cards::Creature>>,

// Bad
attackers: Query<(Entity, &cards::Controller, &cards::CardDefinition)>
// ... in the body:
if !definition.type_line.types.contains(&cards::CardType::Creature) { continue; }
```

(The names above land in later phases; the example shows the target shape.)

### Smells

- Rule reads `definition.keywords` / `definition.types` / `effective_power_toughness` → read the derived marker / `EffectivePT`.
- `if !… { continue }` testing a component's presence or a zone → put it in the query filter.
- Insert/remove of a derived component outside layers → fix: move the write into `r613_1_layers`.
- An effect mutates characteristics directly instead of spawning a `ContinuousEffect` → fix: spawn the effect entity.
- A rule copies the zone-lookup query → fix: use the `ZoneLookup` SystemParam.
- An observer changes game state for a CR rule → fix: emit a message and let the rule system handle it.
- Reading a card that already left its zone through the live entity → fix: use LKI.
- A rule reads derived components where layers may be stale → fix: recompute layers first or read sources.

## 6. Tests: make an existing one red (rules-style §15)

Red first, then green, but reuse. In this order:
1. an existing test of the rule, with one more assert or step;
2. an existing test of a neighbouring rule that already reaches the situation;
3. a new test, only if 1 and 2 can't show it. Say in the report which tests you checked.

```rust
// Good: the situation is already there, one step makes the test red
    game.play([p1.declare_attackers([bears])])?;
    assert!(game.is_attacking(bears));
    assert!(game.is_tapped(bears));              // new: 508.1f

// Bad: a second test that repeats the whole setup to check one more thing
    #[test]
    fn attacking_taps_the_creature() -> TestResult { /* same 10 lines */ }
```

RUN it red on the old code. A refactor adds no test: players see the same, and an assert on internal
bookkeeping becomes one on what players see. A refactor that uncovers a CR bug becomes a fix:
its own commit, red first. Commit only on the full suite
(`cargo nextest run ... --no-fail-fast`): `just test` stops at the first failure.

## 7. Card fixtures read like oracle text

Card fixtures use the testing prelude and bare DSL names; filters, effects and
conditions combine with `&` / `|` / `!`, never with methods (RATIONALE §6, SMELLS).

## Adding rules here

Add a section per decision the user makes about how verdigris code should look: a one-line
rule, a Good/Bad example, and (if it is not obvious) a **Why**. Keep general Rust taste in
`lele-syntax-rs` SMELLS.md instead.
