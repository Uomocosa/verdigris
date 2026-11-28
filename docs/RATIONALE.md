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

**7. Pronouns follow the oracle: "you" → `YOU`, "that player" / "it" → the event role.**
When the oracle text says "you", the fixture writes `YOU`; when it says "that player"
or "it", the fixture writes the event's role (`Draw::PLAYER`,
`PutIntoGraveyard::OBJECT`).

```rust
// Good: "If you would draw a card while your library has no cards in it, you win
// the game instead" (Laboratory Maniac).
if_you().would::<DrawFromEmptyLibrary>().instead(YOU.win_the_game()),
// Bad: spelling out the role where the oracle says "you".
if_you().would::<DrawFromEmptyLibrary>().instead(win_the_game(DrawFromEmptyLibrary::PLAYER)),
```

**8. Replacement effects ("If … would …, … instead") read `if … would … instead`.**
The only form is `if_you()` / `if_a_player()` / `if_an_opponent()` / `if_a(filter)`
→ `.would::<E>()` → optional `.when(..)` / `.to(..)` / `.by(..)` → `.instead(..)`.
Each event's `HOLDER` is the subject of "If X would …", so the common case names no
holder. A bare template is mandatory; only `may(..)` makes it optional.

```rust
// Good: "If a player would draw two or more cards, instead that player loses that
// much life" — recipient is the event role, condition is the oracle's "two or more".
if_a_player().would::<Draw>()
    .when(Draw::AMOUNT.is(Value(2).or_more()))
    .instead(Draw::PLAYER.lose_life(Draw::AMOUNT)),
// Bad: the old four-argument form, removed in favour of the sentence above.
all(player()).instead(Draw, Draw::PLAYER, Draw::AMOUNT.is(Value(2).or_more()), vec![..]),
```

**9. `card()` is a card in any zone, never a token; ownership off the battlefield is `owned_by`.**
`card()` matches `has::<CardDefinition> & !token()` (CR 111.1, 108.1). Outside the
stack and battlefield a card has no controller, only an owner (CR 400.3, 109.4), so
ownership questions use `owned_by`, never `controlled_by`.

```rust
// Good: "If a card would be put into an opponent's graveyard from anywhere, exile
// it instead" (Leyline of the Void) — the card goes to its owner's graveyard.
if_a(card() & owned_by(OPPONENT)).would::<PutIntoGraveyard>()
    .instead(PutIntoGraveyard::OBJECT.exile()),
// Bad: the graveyard owner is not whoever controls the card today.
if_a(card() & controlled_by(OPPONENT)).would::<PutIntoGraveyard>()
    .instead(PutIntoGraveyard::OBJECT.exile()),
```

**10. Card restrictions take no `RestrictionId`; `card!` derives `card/kind/event`.**
`cant::<E>()` / `restriction::<E>()` stamp `kind` and `event`; `card!` fills in the
card name from the fixture module. Two restrictions with the same kind and event on
one card are a compile error asking for a manual ID. Rule IDs stay hand-written
through `RestrictionId::rule("…")`.

```rust
// Good: resolves to `test_safe_haven/cant/701.8a/destroy` and siblings.
all(creature() & controlled_by(YOU)).cant::<Destroy>(Destroy::PERMANENT),
// Bad: hand-written ID in a fixture.
all(creature() & controlled_by(YOU)).cant::<Destroy>(
    Destroy::PERMANENT,
    RestrictionId("test/test-safe-haven"),
),
// Rule IDs stay hand-written: `RestrictionId::rule("509.1a/untapped")`.
```

**11. A fixture's text matches the card's real oracle text.**
Never add an ability the card lacks; each fixture line checks against the printed
card line by line.

```rust
// Good: Laboratory Maniac is one replacement effect, nothing else.
card! {
    cards::creature("Laboratory Maniac", cost![2, U], pt!(2, 2))
        .subtype(cards::HUMAN)
        .subtype(cards::WIZARD);
    if_you().would::<DrawFromEmptyLibrary>().instead(YOU.win_the_game()),
}
// Bad: a stray `begin_the_game_with_it_on_the_battlefield()` on Laboratory Maniac
// — that line belongs to Leyline of the Void, not to this card.
```
