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

- Card `Restrict` conditions are translated per queried event: each `*_program()` takes
  `answers` and the event, and `PlayDatalog::card_restriction_rules`
  (`crates/frogify/__methods__/play_datalog/card_restriction_rules.rs`) turns every
  `Restrict` from `holder_changes` into a `violates` rule from the bound condition, the same
  shape as the base-restriction rules from `PlayDatalog::base_restriction_rules`
  (`crates/frogify/__methods__/play_datalog/base_restriction_rules.rs`), which holds the
  per-base-restriction loop (translate, `violates` rule with `not waived` when `!unwaivable`,
  `waiver_rules` call) shared by all four programs. Waivers for card ids are derived in
  Datalog too: `card_restriction_rules` calls `waiver_rules` for each translated card
  restriction that is `!unwaivable`.
- Still Rust: a card condition the translator rejects (`Untranslatable`, `TooManyConditions`)
  falls back to an `added` fact from `pair_facts_for` (`translatable_restriction` gates it, so a
  translatable condition never double-reports). No current test card hits the fallback: every
  `Restrict` on the four Datalog events translates (flying `702.9b/blockers`, defender
  `702.3b/defender`, Timid Field, Sky Court); the other card `Restrict`s target events with no
  Datalog program (hexproof, indestructible, Silent Arbiter on `DeclareAttackers`, Frozen Relic
  on `Untap`, Platinum Angel on win/lose). `Count`/`ArgAmount` are pre-folded by `bind(event)`
  in Rust before translation.
- `Allow` whens / `AsThough` are derived in Datalog per queried event:
  `PlayDatalog::waiver_rules`
  (`crates/frogify/__methods__/play_datalog/waiver_rules.rs`) turns every `AsThough` for a
  `Restriction(id)` into `waived(ev, C, P, id) :- candidate`, and every `Allow` whose bound
  `when` translates into `waived(ev, C, P, id) :- candidate, root`. Still Rust: an `Allow`
  whose `when` the translator rejects (`Untranslatable`, `TooManyConditions`) falls back to a
  `waived` fact from `pair_facts_for` when `PlayDatalog::untranslated_waiver` (same file) finds
  the `when` holds; no current oracle card hits that fallback (checked once with a temporary
  `eprintln`, removed: zero hits over all 48 oracles, e.g. Crucible's `InZone(Graveyard)`
  translates since F5). The card-side `added` veto rule carries `not waived(ev, C, P, I)`, so
  an untranslatable restriction never double-reports when a waiver holds; `added` facts carry
  no unwaivable flag because none is needed — waiver rules and `waived` facts are never
  produced for `unwaivable` ids, so the guard is vacuous for them. `PlayDatalog::waived()`
  (`crates/frogify/__methods__/play_datalog/waived.rs`) is deleted: it has no caller left.
- Legality is still reached through the rule engine's own `Rules::legality`, not through a
  `Legality` trait the rule engine defines: `PlayDatalog::legality`
  (`crates/frogify/__methods__/play_datalog/legality.rs:12-61`) takes `&rules::Rules` directly,
  and there is no `Legality` trait anywhere in the rule engine.
- Tests compare against that Rust oracle: the `legality` tests read the expected answer from
  `answers.legality(..)` (`crates/frogify/__methods__/play_datalog/legality.rs:126,160,197,240,281`)
  and assert frogify agrees, so a divergence in the shared Rust evaluation would move both sides together.
