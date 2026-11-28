# RATIONALE — card definitions and domains (verdigris-specific)

Extends the fleet `lele-syntax-rs` RATIONALE; it does not replace it. Read both before
writing or reviewing card definitions or adding domain items.

**1. Cards read like the printed card: one `card! { base; abilities }` block.**
One base card, then one ability per line, in printed order, so the definition can be
checked against the Oracle text line by line.

```rust
card! {
    cards::creature("Soul Warden", cost![W], pt!(1, 1));
    abilities::whenever(ANOTHER_CREATURE.enters(), YOU.gain_life(1)),
}
```

**2. A domain is named after what its items return.**
Each domain owns one return type, so there is exactly one place to look for each kind
of item: `cards` (base card + battlefield markers like `cards::Flying`), `abilities`
(`Ability`, `TriggeredAbility`, `Trigger`, `TriggerEvent`, `Condition`,
`whenever`/`at`/`when`), `keywords` (`StaticKeyword` + `keywords::flying()` etc.),
`effects` (what happens on resolution), `turns` (steps, phases, timing:
`turns::beginning_of`).

```rust
abilities::at(
    turns::beginning_of(UPKEEP, YOU).if_(YOU.has_life_at_least(40)),
    effects::win_the_game(),
)
```

**3. Fully qualified names everywhere, including inside `card!`; `card!` is layout only.**
No glob imports, so every item carries its domain on its sleeve. The macro
(`crates/rule-engine/src/cards/__basic__/macros.rs:48`) expands to
`.abilities([Ability::from(..), ..])` — it adds no semantics.

```rust
// Good: domain visible on every item.
card! {
    cards::creature("Felidar Sovereign", cost![4, W, W], pt!(4, 6));
    keywords::vigilance(),
}
```

**4. Test card tests compare the definition against the full explicit struct.**
The test rebuilds the expected `TriggeredAbility` field by field, which proves the
builder output instead of echoing it back
(`crates/rule-engine/src/testing/__fixtures__/cards/soul_warden.rs:40`).

```rust
assert_eq!(
    warden.triggered_abilities,
    [abilities::TriggeredAbility {
        event: ANOTHER_CREATURE.enters(),
        intervening_if: None,
        effect: YOU.gain_life(1),
    }]
);
```

**5. CardDefinition mirrors the printed card.**
A card is stored as the five things printed on it: its name, mana cost (CR 202), type line
(205 — supertypes, types, subtypes), text box (207 — `text: Vec<Ability>` in printed order)
and power/toughness (208). `Ability` has only the CR 113.3 categories: `Spell` (113.3a),
`Triggered` (113.3c) and `Static` (113.3d) today, `Activated` (113.3b) when a card needs one.
Nothing else is a top-level card field.

```rust
pub struct CardDefinition {
    pub name: cards::CardName,
    pub cost: cards::Cost,
    pub type_line: cards::TypeLine,
    pub power_toughness: Option<cards::PowerToughness>,
    pub text: Vec<abilities::Ability>, // printed order
}
```

Everything else lives at the level the CR gives it. Keywords (702) are static abilities in
`StaticAbility::Keyword`, not a separate `keywords` field or an `Ability` category of their own.
"Enters tapped" is a static replacement ability (`StaticAbility::Replacement`, 614.1c), not a
bool. An Aura's "Enchant creature" is the `Enchant` keyword (702.5) and its "Enchanted creature
gets +1/+2" is a continuous ability on the enchanted object (303.4); there is no `aura` field.
"Attacks/blocks each combat if able" are continuous rule abilities (`Change::Rule`), not
keywords. A basic land's "{T}: Add {G}" is not printed at all: rule 305.6 derives it from the
land subtype.

**Why:** the card text maps 1:1 to the struct, so a definition can be checked line by line
against the Oracle text. The CR is the spec: every field and every `Ability` variant has a
printed counterpart or a CR rule that owns it, and no field exists without one.

**6. Card fixtures read like oracle text.**
This refines §3 for fixtures
(`crates/rule-engine/src/testing/__fixtures__/cards/`): the prelude carries the domains,
so the fixture reads like the printed card. A fixture starts with
`use crate::testing::prelude::*;` and uses DSL names bare; the only prefix left is
`cards::` on the card builder (`cards::creature(...)`,
`cards::Supertype::Legendary`), because the bare `creature()` is the filter. Filters,
effects and conditions combine with operators, never with methods: `a & b`, `a | b`,
`!a`. Card text stays as close as possible to the oracle text: repeat a condition
rather than binding it with `let`. A name a fixture needs goes into
`crates/rule-engine/src/prelude/mod.rs`, listed by name; testing-only items go into
`src/testing/prelude/mod.rs`. A name clash is decided by the user.

```rust
// crates/rule-engine/src/testing/__fixtures__/cards/always_watching.rs:1-9
use crate::testing::prelude::*;

card! {
    cards::CardDefinition::from_enchantment("Always Watching", cost![1, W, W]);
    all(creature() & controlled_by(YOU) & !token()).get_pt(1, 1),
    all(creature() & controlled_by(YOU) & !token()).gain(vigilance()),
}
```

**Why:** shorter code reads clearer.
