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
    if players.has_anyone_won() {
        return;
    }
    for player in players.still_in_the_game() {
        if players.have_all_opponents_left(player) {
            wins.write(rules::Proceeds(rules::WinGame(player)));
        }
    }
}
```

## 1. The body reads like the CR sentence

Read the quoted text, then the body: the same words in the same order. "A player still in the
game" is `for player in players.still_in_the_game()`; "opponents have all left the game" is
`players.have_all_opponents_left(player)`; "wins the game" is `WinGame(player)`.

**Why:** the rule can be checked against the CR line by line, and it stays correct when the
helper learns more (teams, players who left), because the rule never spelled out the mechanism.

## 2. Ask named questions, don't compute them

A CR concept is a method on a read-only `SystemParam`, named with the CR's words:
`players::Players`, `rules::Rules`, `zones::ZoneLookup`. A `SystemParam` only answers; the rule
body does the writing.

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
// Bad — "has not lost" is only part of "still in the game"
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
players.have_all_opponents_left(player)
```

A guard that has a CR rule behind it stays: `has_anyone_won()` is 104.1 ("a game ends immediately
when a player wins").

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

Only the system that performs an event writes its result. `r104_2_win_the_game` is the only one
that inserts `Won`, `r104_3_lose_the_game` the only one that inserts `Lost`; every other rule
sends them a message.

**Why:** a trigger or a replacement hooks in at one place, and no path forgets a side effect
(such as `StateBasedActionsPerformed`).

Some older rules do not follow this yet (direct `InZone(graveyard)` inserts, direct `Life`
writes). Don't copy them.

## 7. Players in turn order

Bevy's query order is not turn order. Anything that loops over players, or returns them, goes
through `players::Players`, which reads `turns::TurnOrder`.

## Smells

| Smell | Fix |
|---|---|
| `.filter(..).map(..).collect()` spelling out a CR concept in a rule body | §2 |
| `Without<players::Lost>` in a rule | `players::StillInTheGame`, §3 |
| an early return no CR text explains | §4 |
| `Proceeds` in a rule whose CR text doesn't say it can't be stopped, or `Proceeds` inside a helper | §5 |
| `insert(players::Won)` / `insert(players::Lost)` outside `r104_2` / `r104_3` | send the event, §6 |
| a loop over `Query<Entity, With<players::Player>>` whose order matters | §7 |

## Adding rules here

One section per decision the user makes about how rules look: a one-line rule, a Good/Bad example
and, if it is not obvious, a **Why**. What can be changed by cards goes in
`references/CHANGEABLE_RULES.md`; general code style goes in `verdigris-style`; card definitions go
in `docs/RATIONALE.md` and `docs/SMELLS.md`.
