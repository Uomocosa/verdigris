---
name: verdigris-rules-style
description: Use before adding, changing or reviewing ANY rule system, game action, card effect, static ability, replacement effect or rule-changing card in verdigris. Says how a __rules__ body reads (like the Comprehensive Rules sentence it quotes) and how it acts (Attempt / Resolve, one writer per outcome). Its references/CHANGEABLE_RULES.md defines the event pipeline and MUST be read. Read together with verdigris-style.
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

A missing zone or component is not the announcing rule's problem: the system that performs the event
looks it up (`r110_1_enter_battlefield` finds the battlefield), the rule just writes `Attempt(..)`.

No rule checks "is the game over": 104.1 (`r104_1_game_ends`) stops every `RuleSet`. A guard with a
CR rule behind it stays (800.4a: `if holder.is_still_in_the_game() { return; }`).

## 5. `Attempt` asks, `Resolve` doesn't

`Resolve<E>` is about events; 608 (resolving) is about the stack.

| Write | When |
|---|---|
| `Attempt(event)` | the default: cards may forbid or replace the event |
| `Resolve(event)` | only when the CR says no effect can stop it |
| `hooks.announce(events_in_turn_order)` | "some cards allow ..." (103.6): with no card, nothing happens |

Besides the pipeline itself, the rules that write `Resolve` are 104.2a ("overrides all effects
that would preclude that player from winning") and 104.3a (conceding, the exception named in
101.1). A new one needs a CR quote as clear as those.

Write `Resolve(..)` out in the rule body. Never wrap it in a helper (`declare_as_winner`): a card
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
through `players::Players`, which reads `turns::TurnOrder`. Turn order begins with the starting player,
and `still_in_the_game()` skips players who left, so "each player in turn order" is one `for` over
it: no `loop`, no `next_in_turn_order()` chain, no start / stop test.

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

Only `r500_1` begins a step from `NextStep`; a skip rule takes no `GameMut`.

## 9. A player is a `players::PlayerId`

A field or parameter that is always a player is a `PlayerId`, never an `Entity`. A value that can be
a player or a permanent (a damage recipient, a target, `Context::subject`) stays an `Entity`. Never
wrap an `Entity` in `PlayerId(..)` just to make it compile.

## 10. A rule is a hook point

A rule says WHEN something would happen and to whom; cards decide WHAT is different. So the rule
announces `Attempt(event)` and stops there. If no card reacts, the performing system does the CR's
default, and **the default may be nothing at all** (then nothing reads `Resolve`).

A rule never looks for cards that react: no query over abilities, no "does this card have ...", no
list of candidates, no early return when the list is empty. Announce the event for every object or
player the CR sentence names; the pipeline finds who reacts. Moving the search into a `SystemParam`, a
method or a choice's "legal options" is the same search: what a card allows is written on the card as a
change to the event (`CHANGEABLE_RULES.md` §3), so nobody needs the list.

```rust
// the upkeep step (503.1) has no turn-based actions. Bad: the rule knows which cards matter
for card in permanents.iter().filter(|card| card.has_upkeep_effect()) { perform(card); }

// Good: announce it; with no reacting card, nothing happens
if let Some(active_player) = game.active_player() {
    upkeeps.write(Attempt(events::BeginUpkeep(active_player.id)));
}
```

The announcement happens once, at its moment: "once X is complete", "at the beginning of Y" is a message
the rule reads, never a state it polls every frame. What happens after ("then the player takes their
turn", "receives priority") is another sentence, so another rule.

The event is about the object that can change it: one event per object when each object may change
its own (the pipeline applies only the first matching `Instead` to an event).

Rules that queue triggered abilities (603.2, 508.3a, 509.3a) look them up themselves today; this section doesn't cover them.

**Why:** a new card then needs no change in any rule.

## 11. One CR rule, one file; a general sentence names no case

Asking, checking an answer, and performing are different jobs, so they are different systems. When a
general rule has lettered sub-rules (120.3: "damage may have one or more of the following results";
120.3a damage to a player, 120.3e damage to a creature), the general system names none of them, and
each sub-rule has its own file that picks out its own case. Adding a sub-rule adds a file and edits
no other rule. A sub-rule reads only its own event, never the general event's `Resolve` to build a list.

A sub-rule that is only a condition ("a player can't ... unless") is a clause of the event and has no
file; its text is quoted on the rule that owns the event.

**How to check:** delete one sub-rule in your head. If the general system no longer makes sense or
compiles, it named a case.

One file per CR rule. The `pub fn` named after the file is the rule's system or, if the sentence needs
several, it returns them `.chain()`ed; those systems are private in the same file. A different CR rule
is a different file: 502.3 announces the untap, 701.26b says what untapping does. An old combined `rA_rB_..` file is split when it is refactored.

## 12. Who gives the choice

| The choice comes from | Who asks |
|---|---|
| a card or effect: "you may ..." on the card, or a CR sentence that only points at it ("some cards allow ...", "if an effect lets a player ...") | the card's change (`may` in `CHANGEABLE_RULES.md` §3); the rule is a hook point (§10) and has no `Ask` |
| the CR sentence itself, with no card involved ("the player chooses one of them") | the rule writes `Ask(choice)` and reads `Answered<Choice>` |

Test: remove every card from the game. Does the CR sentence still make the player choose? If not, the
choice isn't the rule's. Not every card gives a choice: another one may simply do something at that
moment, and the same hook point must serve both.

Asking, waiting and validating is ONE shared mechanism for the whole engine, not code in a rule: it
shows one ask at a time, in turn order, drops an ask with no options or whose player left, checks
player / legality / count / duplicates, and writes `Answered` or `Rejected`. The choice type alone
says what a legal answer is. A rule body that needs a choice has no `PendingChoice` and no answer check.

```rust
// 701.22a "look at the top N cards ..., then put any number of them on the bottom": the CR asks
for scry in scries.read() { asks.write(actions::Ask(actions::ScryChoice::new(scry))); }

// another system, 701.22a's result: only valid answers arrive here
for answered in answers.read() { .. }
```

For a card's `may` the mechanism exists: `actions::Ask` / `actions::Answered`, shown and checked
by `answer_asks`; `HookEvent::register` registers what `show_may` / `answer_may` need
(`PendingMayAsks` and `AcceptedMays` for `Hooked<E>`), and the systems still go into the rule's
chain (see `add_turn_systems`). For a choice the CR itself gives, rules still write `PendingChoice`:
don't copy them; a refactor of such a rule extends this mechanism and uses it.

Other choice rules may keep their own `PendingChoice` until they are refactored; the old and new way
can live side by side.

Rules that read a `PlayerAction` (play, cast, attack) keep doing so for now.

## 13. State another rule needs is a CR concept

When rule A blocks rules B, C and D, they don't each test A's bookkeeping. Find the CR concept that
already blocks them (usually priority, the step, or the stack) and make THAT true. While a spell
resolves no player has priority (117.2e), so nobody can cast: one fact, zero guards.

This holds from both sides. Refactoring A: remove the gate from B, C and D. Refactoring B: it must not
mention A any more, so plan the change that makes the concept true, even though A is another file.
Moving the test into a method (`game.x_is_complete()`) or into the shared part keeps the gate; it is
removed only when no rule mentions A. If the concept's type can't say it yet (priority always has a
holder, a step always exists), change the type: that is the refactor, not a follow-up. "It touches many call sites" is no reason to
leave the old state and compute the answer from A inside `Game`, `GameMut` or `Rules`: that is the same
gate in three places.

First look at what B already tests. If B has a condition on the concept (a `has_priority` clause, a
step test, an empty-stack clause), make that condition false at the right moments and delete the gate;
do not add a new "has X finished" fact, and do not carry the gate into an announcing or rejecting
system.

A resource that exists only so other rules can ask "are we done yet?" is a smell, and so is a counter
that stands for "everyone did it".

## 14. Reuse first; a method is general or it isn't a method

1. Look for an existing method that answers the question, even if slightly wider (`Players`,
   `PlayerView`, `Game`, `GameMut`, `Rules`, `ZoneLookup`). Compose two before adding one.
2. A new method is named for a CR noun or verb any rule could use ("the player's hand", "next in turn
   order"). If its name contains the rule's own topic, or only this rule would call it, don't add it:
   the sentence's own work (grouping, comparing) stays in the body, written with general methods. This
   settles §2: §2 is for questions several rules ask.
3. **Moving the body into a helper is not a refactor.** If the helper still does the checks, the
   complexity only moved. Remove it: give it to an event's clauses, the pipeline or the shared choice.
4. Repeated struct literals get a constructor that reads as the CR phrase
   (`LoseLife::by(player, amount)`; if the linter wants another name, its name wins).

A missing engine part is not a reason to keep a rule complicated. Name the part, say what else it
would serve, and plan the rule as if it existed. This applies when a rule or a fixture card needs
the part today. Otherwise leave a seam and name what would trigger building it (the pipeline
for skipping steps waits for the first card that skips a step). Moving an old rule to the shapes
of §5, §6, §10-§13
is in scope of refactoring that rule. A plan applies them; "keep it smaller for now" is
not a reason to skip one.

## 15. Tests: make an existing one red

Red first, then green, but reuse. In this order:
1. an existing test of the rule, with one more assert or step;
2. an existing test of a neighbouring rule that already reaches the situation;
3. a new test, only if 1 and 2 can't show it. Say in the report which tests you checked.

```rust
// Good: the situation is already there, one step makes the test red
    game.play([p1.declare_attackers([bears])])?;
    assert!(game.is_attacking(bears));
    assert!(game.is_tapped(bears));              // new: 508.1f

// Bad: a second test that repeats the whole setup to check one more thing
    #[test]
    fn attacking_taps_the_creature() -> TestResult { /* same 10 lines */ }
```

RUN it red on the old code. A refactor adds no test: players see the same, and an assert on internal
bookkeeping becomes one on what players see. A refactor that uncovers a CR bug becomes a fix:
its own commit, red first. Commit only on the full suite
(`cargo nextest run ... --no-fail-fast`): `just test` stops at the first failure.

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

A message compared with ONE fixed value is a moment too: `.run_if(turns::Step::Draw.begins())`
(Good: `r504_1_draw_step`; Bad: `if **step != turns::Step::X { continue; }`).
Only a rule that uses the varying data reads the message. One flat loop with one write
(104.2a, §0) is already clear and stays.

**Why:** the order can be read and checked in one place, next to the CR words that give it.

## 17. A CR rule that skips a step changes NextStep

`AllPlayersPassed -> r500_2 writes NextStep -> skip rules (r103_8a, r508_8) change it ->
r500_1 begins it -> StepStarted`. Never edit `StepStarted` (614.10: a started step can no
longer be skipped).

```rust
// Good — the step never started, and r500_2 names no case
// crates/rule-engine/__rules__/r103_8a_skip_first_draw_step.rs:14
for step in next_step.read() {
    if **step == turns::Step::Draw && game.is_first_turn() && players.is_two_player_game() {
        *step = turns::NextStep(turns::Step::PrecombatMain);
    }
}
```

**Why:** the step never started, and `r500_2` names no case. Skip rules go between r500_2
and r500_1 in `add_step_end_systems`. Not yet through it: `r500_3` (untap to upkeep, next turn).

## Checklist for a rule (or a plan for one)

- Delete the body and read only the CR sentence: does each clause map to one line, and no line is left?
- Is the choice the CR's or a card's (§12)? Is there a gate other rules still mention (§13)?
- Is the data built first, under a name that says its order, and sent in one statement (§16)?
- What does it do if no card in the game cares? That is the only thing it may perform itself.
- Does it name a card, an ability or a single case of a list? Move it to the case's own rule.
- Does it validate, reject, or wait for a player? Move it to the shared part.
- Would the next card or sub-rule edit this file? Then it isn't a hook point yet.

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

## Adding rules here

One section per decision the user makes about how rules look: a one-line rule, a Good/Bad example
and, if it is not obvious, a **Why**. What can be changed by cards goes in
`references/CHANGEABLE_RULES.md`; general code style goes in `verdigris-style`; card definitions go
in `docs/RATIONALE.md` and `docs/SMELLS.md`.
