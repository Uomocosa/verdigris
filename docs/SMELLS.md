# SMELLS — card definitions and domains (verdigris-specific)

Extends the fleet `lele-syntax-rs` SMELLS; it does not replace it. Each smell shows the
bad shape, then the good shape.

**A card definition chaining `.keyword(..)` / `.whenever(..)` instead of `card!`.**
The chain hides the base/abilities split and invites logic between calls.

```rust
// Bad
cards::creature("Soul Warden", cost![W], pt!(1, 1)).whenever(ANOTHER_CREATURE.enters(), YOU.gain_life(1))
// Good
card! {
    cards::creature("Soul Warden", cost![W], pt!(1, 1));
    abilities::whenever(ANOTHER_CREATURE.enters(), YOU.gain_life(1)),
}
```

**A line inside `card!` over 100 chars.**
rustfmt does not format macro bodies, so long lines never get wrapped for you; split
one argument per line with trailing commas (as in
`crates/rule-engine/src/testing/__fixtures__/cards/felidar_sovereign.rs:20`).

```rust
// Bad: one 100+ char line rustfmt will not split.
abilities::at(turns::beginning_of(UPKEEP, YOU).if_(YOU.has_life_at_least(40)), effects::win_the_game()),
// Good
abilities::at(
    turns::beginning_of(UPKEEP, YOU).if_(YOU.has_life_at_least(40)),
    effects::win_the_game(),
),
```

**Two abilities on one line, or abilities out of printed order.**
One ability per line in Oracle order keeps the definition checkable against the card.

```rust
// Bad
keywords::vigilance(), keywords::lifelink(),
// Good
keywords::vigilance(),
keywords::lifelink(),
```

**Logic inside `card!` (`if`, `let`, loops, `match`).**
The body is data only; branching belongs in the engine or a domain constructor.

```rust
// Bad
card! {
    cards::creature("Soul Warden", cost![W], pt!(1, 1));
    if lifegain { abilities::whenever(ANOTHER_CREATURE.enters(), YOU.gain_life(1)) },
}
// Good: pick the data before the block, or add a domain item.
card! {
    cards::creature("Soul Warden", cost![W], pt!(1, 1));
    abilities::whenever(ANOTHER_CREATURE.enters(), YOU.gain_life(1)),
}
```

**A helper in a domain that doesn't match its return type.**
`effects::whenever`, `cards::flying`, and `abilities::beginning_of` each live in the
wrong home; move them to the domain named after what they return.

```rust
// Bad
effects::whenever(ANOTHER_CREATURE.enters(), effects::gain_life(1))
// Good
abilities::whenever(ANOTHER_CREATURE.enters(), effects::gain_life(1))
```

**A new domain that near-duplicates an existing one.**
`timing` vs `turns`, or `keyword` vs `keywords`, splits one concept across two homes;
extend the existing domain instead.

```rust
// Bad: new `timing` domain next to `turns`.
timing::beginning_of(UPKEEP, YOU)
// Good
turns::beginning_of(UPKEEP, YOU)
```

**`use` / glob imports added to shorten names in card definitions.**
Short names hide which domain each item comes from; keep every path fully qualified.

```rust
// Bad
use crate::abilities::*;
whenever(ANOTHER_CREATURE.enters(), YOU.gain_life(1))
// Good
abilities::whenever(ANOTHER_CREATURE.enters(), YOU.gain_life(1))
```

**A test card test weakened to check only a field or two.**
Checking just the name leaves the builder output unproven; assert the full struct.

```rust
// Bad
assert_eq!(*warden.name, "Soul Warden");
// Good
assert_eq!(
    warden.triggered_abilities,
    [abilities::TriggeredAbility {
        event: ANOTHER_CREATURE.enters(),
        intervening_if: None,
        effect: YOU.gain_life(1),
    }]
);
```

**New test cards outside `src/testing/__fixtures__/cards/`.**
Stray fixture locations make cards undiscoverable; every test card lives in
`crates/rule-engine/src/testing/__fixtures__/cards/`.

**A test binding named for its role instead of the card it holds.**
`kept`, `buried`, `attacking`, `blocker`, `dead` say what the scenario expects, not which
permanent the test is about: the binding cannot be grepped by card, and it goes stale as soon as
the assertion changes. Keep the card name as the stem; add a suffix only to tell two of the same
card apart (`_1`/`_2`, `_opponent`, `_blocking`). The zone or the assertion already carries the
role.

```rust
// Bad: which Isamaru is which, and what happens when the assertion changes?
let [kept, buried] =
    game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(ISAMARU_HOUND_OF_KONDA)?;
game.act(p1, actions::Action::ChooseLegendary(kept.into()));
assert!(game.player(p1).cards_in(GRAVEYARD).has(buried));
// Good
let [isamaru_1, isamaru_2] =
    game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(ISAMARU_HOUND_OF_KONDA)?;
game.act(p1, actions::Action::ChooseLegendary(isamaru_1.into()));
assert!(game.player(p1).cards_in(GRAVEYARD).has(isamaru_2));
```

Also good: `bolt`, `bolt_opponent`, `rats_1`, `bears_blocking`. Not a card, so the role stays:
`let [p1, p2] = ..`, `let zone = game.zone_entity(..)`.

**A field or ability variant with no CR counterpart.**
A `CardDefinition` field or an `Ability`/`StaticAbility` variant that neither appears on the
printed card nor belongs to a CR rule is a second source of truth: it must be kept in sync with
the text box, and nothing in the type says which rule owns it. `does_enter_tapped: bool`
duplicates "enters tapped" (614.1c); `aura: Option<Effect>` duplicates "Enchant creature" plus a
continuous ability on the enchanted object (702.5, 303.4); a separate `keywords` vec duplicates
the text box; `Keyword` as its own `Ability` category puts a 702 ability outside 113.3; a
"keyword" absent from CR 702 is just an effect with a misleading name. Parallel bools on one
axis of `Filter` (`is_this_object`, `is_enchanted_object`) also belong here: they make
impossible combinations representable.

```rust
// Bad — fields that repeat the card text or invent categories
struct CardDefinition {
    name: cards::CardName,
    does_enter_tapped: bool,
    aura: Option<effects::Effect>,
    keywords: Vec<keywords::StaticKeyword>,
}

// Good — one text box, each line an ability in its CR 113.3 category
card! {
    cards::CardDefinition::from_enchantment("Holy Strength", cost![W])
        .subtype(cards::EnchantmentType::Aura.into());
    keywords::enchant(CREATURE),       // 702.5, a static ability
    statics::enchanted().get_pt(1, 2), // 303.4, a continuous ability
}
```

**Fix:** model it as a type-line entry (a supertype or subtype under 205), or as an `Ability` of
the right CR 113.3 category — `StaticAbility::Keyword`, `StaticAbility::Replacement` or
`StaticAbility::Continuous`. For filter flags on one axis, use one enum-valued field such as
`object: Option<effects::ObjectRelation>` instead of parallel bools.

**Domain prefixes in a card fixture.**
The testing prelude already carries the domains (RATIONALE §6); a prefix in a fixture is
noise. The only prefix left is `cards::` on the card builder.

```rust
// Bad
statics::all(creature() & controlled_by(YOU)).gain(keywords::vigilance()),
// Good
all(creature() & controlled_by(YOU)).gain(vigilance()),
```

**Word combinators on filters, effects or conditions.**
Filters, effects and conditions combine with `&` `|` `!`, never with methods; words read
like engine code, not card text.

```rust
// Bad
all(creature().and(controlled_by(YOU)).and(not(token()))).gain(vigilance()),
// Good
all(creature() & controlled_by(YOU) & !token()).gain(vigilance()),
```

**A `let` that hides oracle wording.**
Binding a repeated condition gives it a name the card never uses; repeat the condition
the way the oracle text repeats it (as in
`crates/rule-engine/src/testing/__fixtures__/cards/always_watching.rs:8-9`).

```rust
// Bad
let nontoken = !token();
all(creature() & controlled_by(YOU) & nontoken).get_pt(1, 1),
all(creature() & controlled_by(YOU) & nontoken).gain(vigilance()),
// Good
all(creature() & controlled_by(YOU) & !token()).get_pt(1, 1),
all(creature() & controlled_by(YOU) & !token()).gain(vigilance()),
```

**The old four-argument `instead`.**
Replacement effects read as one sentence now (RATIONALE §8); the positional
`instead(event, holder, cond, vec![..])` hid which part was which.

```rust
// Bad
all(player()).instead(Draw, Draw::PLAYER, cond, vec![lose_life]),
// Good
if_a_player().would::<Draw>().when(cond).instead(Draw::PLAYER.lose_life(Draw::AMOUNT)),
```

**`controlled_by` for a graveyard owner.**
Outside the battlefield a card has an owner, not a controller (RATIONALE §9, CR
400.3); asking who controls it names the wrong player.

```rust
// Bad
if_a(card() & controlled_by(OPPONENT)).would::<PutIntoGraveyard>()
    .instead(PutIntoGraveyard::OBJECT.exile()),
// Good
if_a(card() & owned_by(OPPONENT)).would::<PutIntoGraveyard>()
    .instead(PutIntoGraveyard::OBJECT.exile()),
```

**A hand-written `RestrictionId` in a fixture.**
`card!` derives `card/kind/event` from the module and the `cant`/`restriction` call
(RATIONALE §10); a literal ID repeats that and drifts from it.

```rust
// Bad
all(creature() & controlled_by(YOU)).cant::<Destroy>(
    Destroy::PERMANENT,
    RestrictionId("test/test-safe-haven"),
),
// Good
all(creature() & controlled_by(YOU)).cant::<Destroy>(Destroy::PERMANENT),
```

**An ability the oracle text lacks.**
Every fixture line matches a printed line (RATIONALE §11); an extra ability grants
the card something it never had.

```rust
// Bad: Laboratory Maniac does not begin the game on the battlefield.
card! {
    cards::creature("Laboratory Maniac", cost![2, U], pt!(2, 2));
    begin_the_game_with_it_on_the_battlefield(),
    if_you().would::<DrawFromEmptyLibrary>().instead(YOU.win_the_game()),
}
// Good
card! {
    cards::creature("Laboratory Maniac", cost![2, U], pt!(2, 2));
    if_you().would::<DrawFromEmptyLibrary>().instead(YOU.win_the_game()),
}
