# verdigris-rule-datalog

Translates verdigris game legality into Datalog and checks the answers against the real rule engine.

## Goal

Answer "may this player do this, and if not, why?" with Datalog instead of the imperative checks in
`verdigris-rule-engine`, on any engine implementing `frogmyr::traits::Engine`. It aims to agree with the
rule engine on every test state.

This is **one-step legality only**, the same scope as GDL's `legal/2`. Ordering (the stack, replacement effects,
layers) stays in the rule engine: the prior art (Forge, Gleemin) shows those do not fit unordered Datalog facts.

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
`rule_datalog` against `rules::Rules::legality` for every visible card of the priority player, on `DatafrogEngine`.
Status after F1: 21 of 21 candidates agree, and a hidden hand card gives `None`.

## Next

Cast (CR 601), Attack (508.1a), DeclareBlocker (509.1a) and target candidates (`Candidates::Matching`), each with its own
oracle.
