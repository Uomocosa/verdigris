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
