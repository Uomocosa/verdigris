# SMELLS — card definitions and domains (verdigris-specific)

Extends the fleet `lele-syntax-rs` SMELLS; it does not replace it. Each smell shows the
bad shape, then the good shape.

**A card definition chaining `.keyword(..)` / `.whenever(..)` instead of `card!`.**
The chain hides the base/abilities split and invites logic between calls.

```rust
// Bad
cards::creature("Soul Warden", cost![W], pt!(1, 1)).whenever(ANOTHER_CREATURE.enters(), effects::gain_life(1).to_player(YOU))
// Good
card! {
    cards::creature("Soul Warden", cost![W], pt!(1, 1));
    abilities::whenever(ANOTHER_CREATURE.enters(), effects::gain_life(1).to_player(YOU)),
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
    if lifegain { abilities::whenever(ANOTHER_CREATURE.enters(), effects::gain_life(1).to_player(YOU)) },
}
// Good: pick the data before the block, or add a domain item.
card! {
    cards::creature("Soul Warden", cost![W], pt!(1, 1));
    abilities::whenever(ANOTHER_CREATURE.enters(), effects::gain_life(1).to_player(YOU)),
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
whenever(ANOTHER_CREATURE.enters(), gain_life(1).to_player(YOU))
// Good
abilities::whenever(ANOTHER_CREATURE.enters(), effects::gain_life(1).to_player(YOU))
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
        effect: effects::gain_life(1).to_player(YOU),
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
