---
name: verdigris-linter-rules
description: Use before writing or reviewing a `__rules__` file or a rule test in verdigris; lists the verdigris lint rules, including the ones not enforced yet. Read together with verdigris-rules-style and verdigris-style.
---

# verdigris-linter-rules

`ENFORCED` rules fail `just lint` today. `WILL_BE_ENFORCED` rules are followed by hand now; the linter will check them later. No verdigris-lint code is written until the user says go (AGENTS rule 10).

## VRDG001 rule_file_name · ENFORCED

Every `.rs` file directly inside `__rules__/` (except `mod.rs`) is named `r<id>(_r<id>)*_<slug>.rs`. (AGENTS rule 1; rules-style §11.)

## VRDG002 rule_one_system · ENFORCED

Every `__rules__/` file holds exactly one `pub fn` Bevy system named after the file stem. (AGENTS rule 1; rules-style §11.)

WILL_BE_ENFORCED wording: one file per CR rule; the `pub fn` named after the file is the rule's system or returns the `.chain()` of the private systems in the same file (`crates/rule-engine/__rules__/r508_1f_tap_attackers.rs:12`). The linter's logic already accepts this; only its text changes.

How the linter will check it: require one `pub fn` named after the stem, whose body is the system or a `.chain()` of private systems in that file.

## VRDG003 rule_doc_text · ENFORCED

Every `__rules__/` system fn's doc comment quotes the Comprehensive Rules text it implements, exactly. (rules-style §1.)

## VRDG004 rule_registered · ENFORCED

Every `__rules__/` system is declared in `__rules__/mod.rs` and registered on the app outside `__rules__/` as `S::S`. (rules-style §11.)

## VRDG005 rule_tests · ENFORCED

Every `__rules__/` file keeps a `#[cfg(test)] mod tests` with at least one `#[test]` fn named for what it tests; `test_usage` is banned. (verdigris-style §4.)

## VRDG006 rule_placement · ENFORCED

Every rule system lives in `__rules__/`; no fn or file outside `__rules__/` is named like a rule. (AGENTS rule 1; rules-style §11.)

## VRDG007 rule_comments · ENFORCED

Every `__rules__/` file holds only its system fn's doc comment; every other comment is a violation. (rules-style §1; lele-syntax-rs "no comments".)

## VRDG008 rule_test_shape · WILL_BE_ENFORCED

MockGame setup, then card lookups, then actions and asserts. A lookup after the first `game.play` / `game.act` of a card named in the setup is an error (an end-state check uses `.has(card)`); tokens may be looked up later; a test with no cards needs no lookup. (verdigris-style §3; SMELLS S10.)

Bad — `git show fb680a2:crates/rule-engine/__rules__/r608_2_r608_2c_resolve.rs`:

```rust
game.play([p1.cast(bolt).targeting(bears).end_turn(), p2.end_turn()])?;
assert!(game.player(p2).cards_in(GRAVEYARD).has(bears));
assert!(game.player(p2).cards_in(GRAVEYARD).has(GRIZZLY_BEARS));
let [bolt] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(LIGHTNING_BOLT)?;
assert!(game.player(p1).cards_in(GRAVEYARD).has(bolt));
```

Good — `crates/rule-engine/__rules__/r608_2_r608_2c_resolve.rs:478`:

```rust
let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
game.play([p1.cast(bolt).targeting(bears).end_turn(), p2.end_turn()])?;
assert!(game.player(p2).cards_in(GRAVEYARD).has(bears));
assert!(game.player(p1).cards_in(GRAVEYARD).has(bolt));
```

How the linter will check it: flag `exactly::<N>(CARD)?` after the first `game.play` / `game.act` when `CARD` is named in setup; allow tokens and `.has(..)` asserts.

## VRDG009 no_perform_files · WILL_BE_ENFORCED

No `_perform` rule file: the default of an action lives in the CR rule that defines it (502.3 announces, 701.26b untaps). (rules-style §11.)

Bad — `git show fb680a2:crates/rule-engine/__rules__/r502_3_untap_perform.rs`:

```rust
pub fn r502_3_untap_perform(
    mut untaps: MessageReader<rules::Proceeds<rules::Untap>>,
    mut commands: Commands,
) {
    for untap in untaps.read() {
        commands.entity(untap.permanent).remove::<cards::Tapped>();
    }
}
```

Good — `crates/rule-engine/__rules__/r701_26b_untap.rs:10`:

```rust
pub fn r701_26b_untap(
    mut untaps: MessageReader<Resolve<events::Untap>>,
    mut did_untaps: MessageWriter<Did<events::Untap>>,
    mut commands: Commands,
    tapped: Query<(), With<cards::Tapped>>,
) {
    for untap in untaps.read() {
        if tapped.get(untap.permanent).is_ok() {
            commands.entity(untap.permanent).remove::<cards::Tapped>();
            did_untaps.write(Did(**untap));
        }
    }
}
```

How the linter will check it: flag any `__rules__/*_perform.rs`; require the CR rule that names the action to own the system performing its default.

## VRDG010 who_writes_resolve · WILL_BE_ENFORCED

Only the pipeline (614.1a), 104.2a and 104.3a write `Resolve<E>`; every other rule writes `Attempt<E>` (or announces hooks). (rules-style §5–§6; CHANGEABLE_RULES.md §0.)

Bad — a rule resolving its own event (the real line from `git show fb680a2:crates/rule-engine/__rules__/r608_3e_cannot_enter.rs`, with the wrapper changed):

```rust
graveyards.write(Resolve(events::PutIntoGraveyard { object: spell, owner: **owner }));
```

Good — `crates/rule-engine/__rules__/r608_3e_cannot_enter.rs:41`:

```rust
graveyards.write(Attempt(events::PutIntoGraveyard {
    object: spell,
    owner: **owner,
}));
```

How the linter will check it: flag `MessageWriter<Resolve<…>>` in `__rules__/` except `r614_1a_r616_1_replacement_effects.rs`, `r104_2a_wins_when_opponents_left.rs` and `r104_3a_concede.rs`.

## VRDG011 bare_event_stages · WILL_BE_ENFORCED

`Attempt`, `Resolve`, `Did`, `May` are used bare, imported from the crate root (one `use crate::Attempt;` each), never behind a domain prefix; `events::Untap` keeps its domain. (rules-style §0/§5; CHANGEABLE_RULES.md §0.)

Bad — `git show fb680a2:crates/rule-engine/__rules__/r502_3_untap.rs`:

```rust
use crate::rules;

untaps.write(rules::Would(rules::Untap {
    permanent: entity,
    controller: **owner,
}));
```

Good — `crates/rule-engine/__rules__/r502_3_untap.rs:29`:

```rust
use crate::Attempt;

untaps.write(Attempt(events::Untap {
    permanent: entity,
    controller: **owner,
}));
```

How the linter will check it: flag `rules::Would` / `rules::Proceeds` (and any prefixed stage) in `__rules__/`; require `use crate::<Stage>;` wherever a stage is used.

## VRDG012 no_choice_in_rules · WILL_BE_ENFORCED

No `PendingChoice` / `Rejected` in a rule body (rules-style §12). The rules that still have them are exempt until refactored: `r117_3a_r117_3c_r117_3d_priority.rs`, `r117_5_r704_3_state_based_actions.rs`, `r119_4_r119_8_pay_life.rs`, `r305_1_r305_2_r305_3_play_land.rs`, `r305_6_basic_land_mana_ability.rs`, `r508_1a_declare_attackers.rs`, `r509_1a_declare_blockers.rs`, `r601_2a_cast.rs`, `r601_2c_targets.rs`, `r601_2g_mana_abilities.rs`, `r601_2h_pay.rs`, `r603_3_triggers_on_stack.rs`, `r605_3a_mana_abilities.rs`. `r500_3_..` and `r514_3a_..` read `PendingChoice` only to wait for an open choice.

Bad — `git show 65f4217:crates/rule-engine/__rules__/r701_9a_r701_9b_discard.rs`:

```rust
mut rejected: MessageWriter<actions::Rejected>,
mut pending: ResMut<actions::PendingChoice>,
```

Good — `crates/rule-engine/__rules__/r502_3_untap.rs:15-35` (the rule announces; the card or shared ask carries the choice):

```rust
pub fn r502_3_untap() -> ScheduleConfigs<ScheduleSystem> {
    untap.run_if(turns::Step::Untap.begins())
}

fn untap(
    mut untaps: MessageWriter<Attempt<events::Untap>>,
    game: Game,
    tapped: Query<(Entity, &cards::Owner), With<cards::Tapped>>,
) {
    let Some(active_player) = game.active_player() else {
        return;
    };
    for (entity, owner) in &tapped {
        if **owner == active_player.id {
            untaps.write(Attempt(events::Untap {
                permanent: entity,
                controller: **owner,
            }));
        }
    }
}
```

How the linter will check it: scan each rule body (before `#[cfg(test)]`) for `PendingChoice` / `Rejected`, skipping the exempt file list.

## VRDG013 who_reads_resolve · WILL_BE_ENFORCED

Only the rule that performs an event (its default) reads `Resolve<E>`; triggers read `Did<E>`. (rules-style §6/§10; CHANGEABLE_RULES.md §0.) Exception today: `crates/rule-engine/__rules__/r702_15b_lifelink.rs:12` reads `Resolve<events::DealDamage>` though `r120_3` performs damage.

Bad — `git show fb680a2:crates/rule-engine/__rules__/r510_3a_combat_damage_triggers.rs`:

```rust
mut dealt: MessageReader<rules::Proceeds<rules::DealDamage>>,
```

Good — `crates/rule-engine/__rules__/r510_3a_combat_damage_triggers.rs:14`:

```rust
mut dealt: MessageReader<Did<events::DealDamage>>,
```

How the linter will check it: flag `MessageReader<Resolve<…>>` in a rule that does not own that event's default; triggers must read `Did<…>`.
