# verdigris-rules-style: turn order, priority and steps

Part of `../SKILL.md`; section numbers are shared with it.

## 7. Players in turn order

Bevy's query order is not turn order. Anything that loops over players, or returns them, goes
through `players::Players`, which reads `turns::TurnOrder`. Turn order begins with the starting player,
and `still_in_the_game()` skips players who left, so "each player in turn order" is one `for` over
it: no `loop`, no `next_in_turn_order()` chain, no start / stop test.

## 8. `Game` asks, `GameMut` moves the game

`Game` is read-only and any rule can take it. Questions about the turn itself go through
`game.turn()` (`game.turn().next_step()`). `GameMut` is the only writer of `turns::TurnState`; only the priority and turn-structure rules take it, and a system that takes it can't
also take `Game` or `Rules`.

```rust
// 117.3d: "the next player in turn order receives priority"
if let Some(next) = game.priority_holder().next_in_turn_order() {
    game.pass_priority_to(next.id);
}
```

`next_in_turn_order()` skips players who left. Cards don't override it: a card changes the answer,
through the pipeline.

Only `r500_1` begins a step from `NextStep`; a skip rule takes no `GameMut`.

`Game`, `Players` and `Rules` are passed by value, and clippy refuses more than 256 bytes: one query is
32. So data is grouped behind ONE query: a new turn value is a field of `TurnState`, a new player marker a
`Has<..>` field of `players::Status` (never `&Component`: it conflicts with the systems that write it).
Don't add a query per question.

## 17. The steps of a turn are a list; a CR rule that skips a step changes NextStep

`turns::TurnState` holds the steps still to come this turn. `r500_2` proposes `game.turn().next_step()`; only
`r500_3` and `r508_8` still name the step they go to. Adding a step inserts it into the list (500.9), adding a phase
is the event `AddPhase` (`r500_8`, 500.8); `begin_step` drops the steps it passes over. A turn can have several
beginning or combat phases, so "the turn began" is `turns::TurnStarted`, never the untap step (302.6).

```rust
// 510.4 "the phase gets a second combat damage step"
game.add_step_after_this_one(turns::Step::SecondCombatDamage);
```

`AllPlayersPassed -> r500_2 writes NextStep -> skip rules (r103_8a, r508_8) change it ->
r500_1 begins it -> StepStarted`. Never edit `StepStarted` (614.10: a started step can no
longer be skipped).

```rust
// Good — the step never started, and r500_2 names no case
// crates/rule-engine/__rules__/r103_8a_skip_first_draw_step.rs:14
for step in next_step.read() {
    if **step == turns::Step::Draw
        && game.turn().is_first_turn()
        && players.is_two_player_game()
        && let Some(step_after) = game.turn().step_after_next()
    {
        *step = turns::NextStep(step_after); // 500.11: past it, no destination named
    }
}
```

**Why:** the step never started, and `r500_2` names no case. Skip rules go between r500_2
and r500_1 in `add_step_end_systems`. Not yet through it: `r500_3` (untap to upkeep, next turn).

