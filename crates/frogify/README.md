# frogify

Translates verdigris game legality into Datalog and checks the answers against the real rule engine.

## Goal

frogify is the legality answer for verdigris: "may this player do this, and if not, why?"
It is built from the rule engine's own restrictions and the cards' `Changes` translated to
Datalog (cards as data), with the snapshot giving only raw game state. The end goal: the rule
engine asks frogify (through a `Legality` trait the rule engine defines) instead of its own
one-step legality checks; the same evaluation also answers "all legal actions" at once for UI
highlights and bot/AI move lists. Ordering (the stack, replacement effects, triggers, layers)
stays in the rule engine.

## Current coverage: `Play` (CR 305)

`PlayDatalog` provides:

1. `snapshot(&Rules, viewer)`: the ground facts Play needs, built from what the viewer can see (public zones plus
   their own hand): objects, zones, owners, card types after layers, players, priority, active player, step, empty
   stack, land plays available, and the waivers or extra restrictions cards hold.
2. `play_program()`: Play's 8 restrictions as Datalog rules, translated from the rule engine's own
   `rules::Condition`s. The translator handles exactly the atoms Play uses; anything else is a
   `TranslationError::Untranslatable`, never a silent true or false.
3. `legality(engine, ...)` returns `Option<rules::Legality>`: `None` when the card is not a Play candidate for the
   viewer (for example a card in another player's hand: the snapshot cannot see it, so it is never reported as legal);
   otherwise it reads the derived `violates(play, card, player, id)` facts back as the same `rules::Legality` the rule
   engine returns, with reasons in the order of Play's restriction list.

```prolog
violates(play, C, P, "305.1/land-card") :- play_candidate(C, P), not card_type(C, land),
                                           not waived(play, C, P, "305.1/land-card").
```

Restriction ids come from the rule engine's `events::play::*` constants, so they are never duplicated as strings.

## The oracle

Tests replay the real r305 `MockGame` scenarios (read through the testing-only `MockGame::read_rules`) and compare
frogify against `rules::Rules::legality` for every visible card of the priority player.
Status after F1: 21 of 21 candidates agree, and a hidden hand card gives `None`.

## Next

Cast (CR 601), Attack (508.1a), DeclareBlocker (509.1a) and target candidates (`Candidates::Matching`), each with its own
oracle.

## Temporary — must be resolved before merging to main

What is still done in Rust instead of Datalog today:

- Card `Restrict` conditions are evaluated eagerly by `pair_facts_for` and grounded as `added`
  facts (`crates/frogify/__methods__/play_datalog/pair_facts_for.rs:33-47`): the `!answers.eval(..)`
  check at line 36 runs the restriction's condition in Rust, so only restrictions that already
  hold reach the Datalog program. The program never sees the `Restrict` condition itself.
- `Allow` whens / `AsThough` are grounded by `waived()` as `waived` facts
  (`crates/frogify/__methods__/play_datalog/waived.rs:15-21`): the `answers.eval(..)` call at
  line 18 evaluates each `Allow`'s `when` in Rust, and `AsThough` is always true, so the program
  only ever reads pre-decided `waived` facts, never the waiver conditions.
- Legality is still reached through the rule engine's own `Rules::legality`, not through a
  `Legality` trait the rule engine defines: `PlayDatalog::legality`
  (`crates/frogify/__methods__/play_datalog/legality.rs:12-61`) takes `&rules::Rules` directly,
  and there is no `Legality` trait anywhere in the rule engine.
- Tests compare against that Rust oracle: the `legality` tests read the expected answer from
  `answers.legality(..)` (`crates/frogify/__methods__/play_datalog/legality.rs:126,160,197,240,281`)
  and assert frogify agrees, so a divergence in the shared Rust evaluation would move both sides together.
