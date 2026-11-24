---
name: verdigris-rules-style
description: Use before adding, changing or reviewing ANY rule system, game action, card effect, static ability, replacement effect or rule-changing card in verdigris. Says how a __rules__ body reads (like the Comprehensive Rules sentence it quotes) and how it acts (Would / Proceeds, one writer per outcome). Its references/CHANGEABLE_RULES.md defines the event pipeline and MUST be read. Read together with verdigris-style.
---

# verdigris-rules-style

## 0. MUST read first

1. `references/CHANGEABLE_RULES.md`, **in full**: events, the pipeline, `RuleChange`.
2. Load `verdigris-style` (it lists the other skills and files to load).

The example all sections refer to is `crates/rule-engine/__rules__/r104_2a_wins_when_opponents_left.rs`:

```rust
/// 104.2a A player still in the game wins the game if that player’s opponents have all left the
/// game. This happens immediately and overrides all effects that would preclude that player from
/// winning the game.
pub fn r104_2a_wins_when_opponents_left(
    mut wins: MessageWriter<rules::Proceeds<rules::WinGame>>,
    players: players::Players,
) {
    for player in players.still_in_the_game() {
        if player.have_all_opponents_left() {
            wins.write(rules::Proceeds(rules::WinGame(player.id)));
        }
    }
}
```

## 1. The body reads like the CR sentence

Read the quoted text, then the body: the same words in the same order. "A player still in the
game" is `for player in players.still_in_the_game()`; "opponents have all left the game" is
`player.have_all_opponents_left()`; "wins the game" is `WinGame(player.id)`.

**Why:** the rule can be checked against the CR line by line, and it stays correct when the
helper learns more (teams, players who left), because the rule never spelled out the mechanism.

## 2. Ask named questions, don't compute them

A CR concept is a method on a read-only `SystemParam`, named with the CR's words:
`Game`, `players::Players`, `rules::Rules`, `zones::ZoneLookup`. A `SystemParam` only answers; the
rule body does the writing (the one exception is `GameMut`, §8).

A question about one player is a method on the player handle `players::PlayerView`. Rules get a
handle from a named question (`players.still_in_the_game()`, `game.active_player()`,
`game.priority_holder()`), never from an id: `Players::view` is not for rules.

```rust
// Bad — who is this, and why is it passed?
players.opponents_still_in_the_game(turn.active)

// Good — the receiver is named with the CR's word (506.2: the active player is the attacking player)
let attacking_player = game.active_player();
attacking_player.opponents_still_in_the_game()
```

(`active_player()` is an `Option`: after the active player leaves there is none, 800.4j.)

```rust
// Bad — the reader must decode what the chain means
let still_in_the_game: Vec<Entity> =
    players.iter().filter(|(_, lost)| lost.is_none()).map(|(player, _)| player).collect();

// Good
players.still_in_the_game()
```

Search the existing `SystemParam`s before adding a method. Add one only if it hides real work or
several rules ask the same question (lele-syntax-rs SMELLS S5).

## 3. A CR phrase in a query is a named filter

When a rule needs other components too, filter with a type alias named after the CR phrase.

```rust
// Bad — "has not lost" is not "still in the game" (104.5: losing players leave it)
players: Query<(Entity, &players::Life), Without<players::Lost>>,

// Good
players: Query<(Entity, &players::Life), players::StillInTheGame>,
```

## 4. No guards the CR doesn't have

An early return with no CR counterpart usually means the question was asked wrongly. Ask it the
way the CR does and the guard disappears.

```rust
// Bad — nothing in 104.2a talks about the number of players
if players.iter().count() < 2 { return; }

// Good — true only if the player has opponents and all of them left
player.have_all_opponents_left()
```

No rule checks "is the game over": 104.1 (`r104_1_game_ends`) stops every `RuleSet`. A guard with a
CR rule behind it stays (800.4a: `if holder.is_still_in_the_game() { return; }`).

## 5. `Would` asks, `Proceeds` doesn't

| Write | When |
|---|---|
| `Would(event)` | the default: cards may forbid or replace the event |
| `Proceeds(event)` | only when the CR says no effect can stop it |

Besides the pipeline itself, the rules that write `Proceeds` are 104.2a ("overrides all effects
that would preclude that player from winning") and 104.3a (conceding, the exception named in
101.1). A new one needs a CR quote as clear as those.

Write `Proceeds(..)` out in the rule body. Never wrap it in a helper (`declare_as_winner`): a card
effect would reuse the helper and bypass cards like Platinum Angel.

## 6. One writer per outcome

Only the system that performs an event writes its result; every other rule sends it a message.

| Outcome | Only writer |
|---|---|
| `Won` / `Lost` | `r104_2_win_the_game` / `r104_3_lose_the_game` |
| `GameIsADraw` / `LeftTheGame` | `r104_4a_draw` / `r104_5_leaves_the_game` |
| gaining, losing or paying life | `r119_3_life_loss`, through `GainLife` / `LoseLife` |
| `Turn`, `Priority` | `GameMut` (§8) |

**Why:** a trigger or a replacement hooks in at one place, and no path forgets a side effect
(such as `StateBasedActionsPerformed`).

Some older rules do not follow this yet: destroy, counter, discard, mill and finished spells still
insert `InZone(graveyard)` themselves instead of sending `PutIntoGraveyard`. Don't copy them.

## 7. Players in turn order

Bevy's query order is not turn order. Anything that loops over players, or returns them, goes
through `players::Players`, which reads `turns::TurnOrder`.

## 8. `Game` asks, `GameMut` moves the game

`Game` is read-only and any rule can take it. `GameMut` is the only writer of `Turn` and
`Priority`; only the priority and turn-structure rules take it, and a system that takes it can't
also take `Game` or `Rules`.

```rust
// 117.3d: "the next player in turn order receives priority"
if let Some(next) = game.priority_holder().next_in_turn_order() {
    game.pass_priority_to(next.id);
}
```

`next_in_turn_order()` skips players who left. Cards don't override it: a card changes the answer,
through the pipeline.

## 9. A player is a `players::PlayerId`

A field or parameter that is always a player is a `PlayerId`, never an `Entity`. A value that can be
a player or a permanent (a damage recipient, a target, `Context::subject`) stays an `Entity`. Never
wrap an `Entity` in `PlayerId(..)` just to make it compile.

## 10. One check, one red test

A behaviour change starts with a test that was RUN and failed on the old system. Commit only on the
full suite (`cargo nextest run ... --no-fail-fast`): `just test` stops at the first failure.

## Smells

| Smell | Fix |
|---|---|
| `.filter(..).map(..).collect()` spelling out a CR concept in a rule body | §2 |
| `Without<players::Lost>` in a rule | `players::StillInTheGame`, §3 |
| an early return no CR text explains | §4 |
| `Proceeds` in a rule whose CR text doesn't say it can't be stopped, or `Proceeds` inside a helper | §5 |
| a `Won` / `Lost` / `GameIsADraw` / `LeftTheGame` insert or a `Life` write outside its writer | send the event, §6 |
| `ResMut<turns::Turn>` or `ResMut<turns::Priority>` in a rule | `GameMut`, §8 |
| an id passed to ask about a player (`players.x(turn.active)`) | a handle, §2 |
| a count of players or passes compared with another count | ask about the players themselves, §2 |
| a loop over `Query<Entity, With<players::Player>>` whose order matters | §7 |

## Adding rules here

One section per decision the user makes about how rules look: a one-line rule, a Good/Bad example
and, if it is not obvious, a **Why**. What can be changed by cards goes in
`references/CHANGEABLE_RULES.md`; general code style goes in `verdigris-style`; card definitions go
in `docs/RATIONALE.md` and `docs/SMELLS.md`.
