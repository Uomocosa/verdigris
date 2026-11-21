# RATIONALE — card definitions and domains (verdigris-specific)

Extends the fleet `lele-syntax-rs` RATIONALE; it does not replace it. Read both before
writing or reviewing card definitions or adding domain items.

**1. Cards read like the printed card: one `card! { base; abilities }` block.**
One base card, then one ability per line, in printed order, so the definition can be
checked against the Oracle text line by line.

```rust
card! {
    cards::creature("Soul Warden", cost![W], pt!(1, 1));
    abilities::whenever(ANOTHER_CREATURE.enters(), effects::gain_life(1).to_player(YOU)),
}
```

**2. A domain is named after what its items return.**
Each domain owns one return type, so there is exactly one place to look for each kind
of item: `cards` (base card + battlefield markers like `cards::Flying`), `abilities`
(`Ability`, `TriggeredAbility`, `Trigger`, `TriggerEvent`, `Condition`,
`whenever`/`at`/`when`), `keywords` (`Keyword` + `keywords::flying()` etc.),
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
        effect: effects::gain_life(1).to_player(YOU),
    }]
);
```
