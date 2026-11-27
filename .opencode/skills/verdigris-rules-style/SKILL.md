---
name: verdigris-rules-style
description: Use before adding, changing or reviewing ANY rule system, game action, card effect, static ability, replacement effect or rule-changing card in verdigris. Says how a __rules__ body reads (like the Comprehensive Rules sentence it quotes) and how it acts (Attempt / Resolve, one writer per outcome). Its references/CHANGEABLE_RULES.md (event pipeline) and references/ACTING.md (what a rule writes) MUST be read; references/TURNS.md for turn, priority and step rules. Read together with verdigris-style.
---

# verdigris-rules-style

## 0. MUST read first

1. `references/CHANGEABLE_RULES.md`, **in full**: events, the pipeline, `RuleChange`.
2. `references/ACTING.md`, **in full**: what a rule writes and who performs it (§5, §6, §10, §12, §13).
3. `references/TURNS.md` when the rule touches turn order, priority or steps (§7, §8, §17).
4. Load `verdigris-style` (it lists the other skills and files to load). Tests (§15) are in its §6.

Section numbers are the same in every file, so "rules-style §12" is in ACTING.md.

The example all sections refer to is `crates/rule-engine/__rules__/r104_2a_wins_when_opponents_left.rs`:

```rust
/// 104.2a A player still in the game wins the game if that player’s opponents have all left the
/// game. This happens immediately and overrides all effects that would preclude that player from
/// winning the game.
pub fn r104_2a_wins_when_opponents_left(
    mut wins: MessageWriter<Resolve<events::WinGame>>,
    players: players::Players,
) {
    for player in players.still_in_the_game() {
        if player.have_all_opponents_left() {
            wins.write(Resolve(events::WinGame(player.id)));
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
`game.priority_holder()`), never from an id: `Players::view` is not for rules. For the players an event happens to, ask
`players.affected_by(event)` (still in the game, in turn order; `Event::affected` says who, `r701_9b`).

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

Before adding a method, read §14.

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

A missing zone or component is not the announcing rule's problem: the system that performs the event
looks it up (`r110_1_enter_battlefield` finds the battlefield), the rule just writes `Attempt(..)`.

No rule checks "is the game over": 104.1 (`r104_1_game_ends`) stops every `RuleSet`. A guard with a
CR rule behind it stays (800.4a: `if holder.is_still_in_the_game() { return; }`).

## 9. A player is a `players::PlayerId`

A field or parameter that is always a player is a `PlayerId`, never an `Entity`. A value that can be
a player or a permanent (a damage recipient, a target) stays an `Entity`. Never
wrap an `Entity` in `PlayerId(..)` just to make it compile. To ask whether such an `Entity` is a player, use
`players.as_player(entity)`; with an event, `players.affected_by(event)`.

Owner and controller are different players (108.3, 108.4): "you control", who casts, who blocks, a trigger's
controller and hexproof read `cards::Controller`; whose hand, library or graveyard a card goes to reads
`cards::Owner`. An ability on the stack has a controller and no owner.

## 11. One CR rule, one file; a general sentence names no case

Asking, checking an answer, and performing are different jobs, so they are different systems. When a
general rule has lettered sub-rules (120.3: "damage may have one or more of the following results";
120.3a damage to a player, 120.3e damage to a creature), the general system names none of them, and
each sub-rule has its own file that picks out its own case. Adding a sub-rule adds a file and edits
no other rule. A sub-rule reads only its own event, never the general event's `Resolve` to build a list.

A sub-rule that is only a condition ("a player can't ... unless") is a restriction of the event and has no
file; its text is quoted on the rule that owns the event.

**How to check:** delete one sub-rule in your head. If the general system no longer makes sense or
compiles, it named a case.

One file per CR rule. The `pub fn` named after the file is the rule's system or, if the sentence needs
several, it returns them `.chain()`ed; those systems are private in the same file. A different CR rule
is a different file: 502.3 announces the untap, 701.26b says what untapping does. An old combined `rA_rB_..` file is split when it is refactored; the file keeps the id that says what is performed. A sentence that says WHEN and one that says WHAT HAPPENS are two rules (500.4 announces that pools empty, 106.4 performs the loss); an id with no performer of its own (510.2 "dealt simultaneously") goes under `# See also` of the rule that performs it. A keyword sentence that only says "can't" (702.12b indestructible) is a card-side change in its keyword file's `abilities` (`__keywords__/r702_12b_indestructible.rs`), like haste and vigilance, and its text goes under `# See also` of the performing rule; an id whose sentence only conditions or repeats
the other gets no file, its text moves under `# See also` (in a `__rules__` file) or `# Restates` (in
a `__keywords__` file) and the file is renamed. Text under `# See also` / `# Restates` is quoted word for word like the main quote, with no explanation added. Every id in the file name keeps
its quote (the linter checks it); to drop an id, rename the file. Ids that quote the same sentence (510.4,
702.4b, 702.7b) keep the one in the rule that performs it, the others go under `# See also`. Moving an existing test to the file that now owns its behaviour is not a new test. When a file is split, each
sub-rule takes the existing test that shows its case; the general file keeps a test that shows its own
outcome (`r120_3` writes `Did`, so a trigger on damage). A general file that only announces or writes
`Did` is fine, however short.

## 14. Reuse first; a method is general or it isn't a method

1. Look for an existing method that answers the question, even if slightly wider (`Players`,
   `PlayerView`, `Game`, `GameMut`, `Rules`, `ZoneLookup`). Compose two before adding one.
2. A new method is named for a CR noun or verb any rule could use ("the player's hand", "next in turn
   order"). If its name contains the rule's own topic, or only this rule would call it, don't add it:
   the sentence's own work (grouping, comparing) stays in the body, written with general methods. This
   settles §2: §2 is for questions several rules ask.
3. **Moving the body into a helper is not a refactor.** If the helper still does the checks, the
   complexity only moved. Remove it: give it to an event's restrictions, the pipeline or the shared choice.
4. A new `SystemParam` names each query it reads as its own field (`stack::Targeting`), and holds
   `rules::Rules` only to ask `answers.legal(..)`; it never digs data out of `Rules`' queries.
   SystemParams may pass up to 512 bytes (`clippy.toml`); a Bevy handle is moved once per system run.
5. Repeated struct literals get a constructor that reads as the CR phrase
   (`LoseLife::by(player, amount)`; if the linter wants another name, its name wins).

A missing engine part is not a reason to keep a rule complicated. Name the part, say what else it
would serve, and plan the rule as if it existed. This applies when a rule or a fixture card needs
the part today. Otherwise leave a seam and name what would trigger building it (the pipeline
for skipping steps waits for the first card that skips a step). Moving an old rule to the shapes
of §5, §6, §10-§13
is in scope of refactoring that rule. A plan applies them; "keep it smaller for now" is
not a reason to skip one.

## 16. Create the data first, then act on it

A rule body has two parts: build a named list of what would happen, in the order the CR gives, then
send the whole list in one statement. The name says what it holds and its order
(`events_in_turn_order`). Propose this shape first for any rule that needs nested loops or an order.

```rust
// Bad: finding the cards, ordering them and writing are mixed in nested loops
for player in players.still_in_the_game() {
    for card in player.opening_hand() {
        opening_hand_actions.write(Attempt(events::OpeningHandAction(card)));
    }
}

// Good: first the data (103.6: "the starting player ... then each other player in turn order")
fn opening_hand_actions(
    mut hooks: ResMut<rules::HookQueue<events::OpeningHandAction>>,
    players: players::Players,
) {
    let events_in_turn_order: Vec<events::OpeningHandAction> = players
        .still_in_the_game()
        .iter()
        .flat_map(players::PlayerView::opening_hand)
        .map(events::OpeningHandAction)
        .collect();
    hooks.announce(events_in_turn_order);
}
```

With a plain `MessageWriter`, `into_iter().map(Attempt)` on the collected list trips
`clippy::needless_collect`; write `write_batch(events_in_turn_order.iter().copied().map(Attempt))`.

A moment with no data ("once the mulligan process is complete") is a run condition, not a loop over
its messages. The rule's `pub fn` returns it, so the moment stays in the rule's file:

```rust
// Bad: for _ in mulligans_complete.read() { .. }
pub fn r103_6_opening_hand_actions() -> ScheduleConfigs<ScheduleSystem> {
    opening_hand_actions.run_if(on_message::<turns::MulligansComplete>)
}
```

A run condition only changes the shape. FIRST ask what the body does: if a card could change it
(§10: "you don't lose ...", "can't", "instead"), it is an event, and a moment that performs the outcome
itself is still wrong.

```rust
// Bad: tidy, but no card can keep its mana
pub fn r106_4_mana_empties() -> ScheduleConfigs<ScheduleSystem> {
    empty_mana_pools.run_if(on_message::<turns::StepStarted>) // sets every pool to EMPTY
}

// Good: r500_4_mana_pools_empty announces; r106_4_lose_unspent_mana performs
for player in players.still_in_the_game() {
    losses.write(Attempt(events::LoseUnspentMana(player.id)));
}
```

A moment system may take read-only `SystemParam`s. It runs once per frame, however many messages came.

A message compared with ONE fixed value is a moment too: `.run_if(turns::Step::Draw.begins())`
(Good: `r504_1_draw_step`; Bad: `if **step != turns::Step::X { continue; }`).
Only a rule that uses the varying data reads the message. One flat loop with one write
(104.2a, §0) is already clear and stays.

**Why:** the order can be read and checked in one place, next to the CR words that give it.

## Checklist for a rule (or a plan for one)

- Delete the body and read only the CR sentence: does each restriction map to one line, and no line is left?
- Is the choice the CR's or a card's (§12)? Is there a gate other rules still mention (§13)?
- Is the data built first, under a name that says its order, and sent in one statement (§16)?
- What does it do if no card in the game cares? That is the only thing it may perform itself.
- Does it name a card, an ability or a single case of a list? Move it to the case's own rule.
- Does it validate, reject, or wait for a player? Move it to the shared part.
- Would the next card or sub-rule edit this file? Then it isn't a hook point yet.
- Does every event it makes or changes pass CHANGEABLE_RULES §6 (no inferable field, general verb, plural, `cause`)?

## Smells

| Smell | Fix |
|---|---|
| `.filter(..).map(..).collect()` spelling out a CR concept in a rule body | §2 |
| `Without<players::Lost>` in a rule | `players::StillInTheGame`, §3 |
| an early return no CR text explains | §4 |
| `Resolve` in a rule whose CR text doesn't say it can't be stopped, or `Resolve` inside a helper | §5 |
| a `Won` / `Lost` / `GameIsADraw` / `LeftTheGame` insert or a `Life` write outside its writer | send the event, §6 |
| `ResMut<turns::Turn>` or `ResMut<turns::Priority>` in a rule | `GameMut`, §8 |
| an id passed to ask about a player (`players.x(turn.active)`) | a handle, §2 |
| a count of players or passes compared with another count | ask about the players themselves, §2 |
| a loop over `Query<Entity, With<players::Player>>` whose order matters | §7 |
| a query or loop that looks for cards with a certain ability, then acts on them | announce the event, §10 |
| `PendingChoice`, `Rejected`, or an answer check in a rule body | the shared ask / answer, §12 |
| the same gate copied in several rules ("not yet", "already done") | make the CR concept true, §13 |
| a helper that returns "all that this rule needs" | §14.3 |
| nested loops that find, order and write at once; `for _ in some_message.read()` | §16 |
| a rule that polls state every frame to announce something | read the message of that moment, §10 |
| `if **step != X { continue }` | §16 |
| a write to `StepStarted` outside `r500_1`/`r500_3` | §17 |
| a `match` from a step to the step after it, or a resource that says an extra step is due | §17 |
| a moment whose body writes an outcome a card could change | §16, §10 |
| an event field readable from another field's entity (`player`, `controller`, `source`) | CHANGEABLE_RULES §6.1 |
| an event named after one case (`PlayLand`, `GivePoisonCounters`, `CastSpell`) | CHANGEABLE_RULES §6.2 |
| a singular and a plural event for one verb (`DeclareAttacker` + `DeclareAttackers`) | CHANGEABLE_RULES §6.3 |
| a wrapper event that only adds a reason (`TapAttacker` instead of `Tap { cause }`) | CHANGEABLE_RULES §6.4 |
| a state-change event without `cause`, or a cause that is not the producer's `ID` / the rule's own `CAUSE` | CHANGEABLE_RULES §6.4 |
| a `bool` on an event that a component or rule already tracks | CHANGEABLE_RULES §6.5 |

## Adding rules here

One section per decision the user makes about how rules look: a one-line rule, a Good/Bad example
and, if it is not obvious, a **Why**. What can be changed by cards goes in
`references/CHANGEABLE_RULES.md`; general code style goes in `verdigris-style`; card definitions go
in `docs/RATIONALE.md` and `docs/SMELLS.md`.
