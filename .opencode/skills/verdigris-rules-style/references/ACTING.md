# verdigris-rules-style: how a rule acts

Part of `../SKILL.md`; section numbers are shared with it.

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
| losing unspent mana | `r106_4_lose_unspent_mana`, through `LoseUnspentMana` |
| `TurnState` (turn, priority, upcoming steps) | `GameMut` (§8) |

**Why:** a trigger or a replacement hooks in at one place, and no path forgets a side effect
(such as `StateBasedActionsPerformed`).

Two rules writing the same component is the same smell: make one event and one performer.

Some older rules do not follow this yet: counter, mill and finished spells still insert
`InZone(graveyard)` themselves instead of sending `PutIntoGraveyard`. Don't copy them. A direct
write skips every `Cant` / `Instead` on the event, so converting one is usually a CR fix: red first
(destroy skipped "can't be put into a graveyard" until `r701_8a_destroy` sent the event).

Before replacing a direct write with an event, check WHEN the event is performed:
`PutIntoGraveyard` is resolved only in the state-based pass, after `RuleSet::Resolution`. A rule that
others in the same set rely on (608.2b must take the spell off the stack before 608.2c reads it)
can't just send the event; plan the ordering too.

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

Rules that queue triggered abilities (603.2, 508.3a, 509.3a) look the ability up themselves. A shared
lookup is parked by the user: don't propose one; refactor what such a rule does around the lookup. What happened they read from `Did<E>`, never from another rule's `Rejected` (§13).

**Why:** a new card then needs no change in any rule.

## 12. Who gives the choice

| The choice comes from | Who asks |
|---|---|
| a card or effect: "you may ..." on the card, or a CR sentence that only points at it ("some cards allow ...", "if an effect lets a player ...") | the card's change (`may` in `CHANGEABLE_RULES.md` §3); the rule is a hook point (§10) and has no `Ask` |
| the CR sentence itself, with no card involved ("the player chooses one of them") | the rule writes `Ask(choice)` and reads `Answered` |

Test: remove every card from the game. Does the CR sentence still make the player choose? If not, the
choice isn't the rule's. Not every card gives a choice: another one may simply do something at that
moment, and the same hook point must serve both.

Asking, waiting and validating is ONE shared mechanism for the whole engine, not code in a rule: it
shows one ask at a time, in the order they were asked (so a rule asks in turn order), drops an ask whose player left, checks
player / legality / count / duplicates, and writes `Answered` or `Rejected`. The choice type alone
says what a legal answer is. A rule body that needs a choice has no `PendingChoice` and no answer check.

```rust
// 701.9b in a rule: the CR asks ...
asks.write(actions::Ask(actions::Choice::Discard(choice)));

// ... and another private system of the same rule performs; only legal answers arrive here
for answered in answers.read() {
    let actions::Choice::Discard(choice) = &answered.choice else { continue };
    ..
}
```

`actions::Ask(Choice)` / `actions::Answered { choice, answer }`, shown and checked by `show_asks` /
`answer_asks`. A new choice adds a `Choice` variant and one arm in `Choice::player`, `Choice::accepts`
(what is legal) and the two `rejection`s; no rule checks an answer. Good: `r701_9b_choose_discards`,
`r704_5j_legend_rule`, `r507_1_choose_defending_player`. A card's `may` uses the same mechanism.

- An ask written in frame N is shown at the end of frame N and can be answered from frame N+1 (checked in
  the Turn set, so every later set of that frame can read `Answered`).
- The asking system must not write the same ask again in the frame of the answer from state the answer has
  not changed yet. A rule that finds its situation again in every pass (a state-based check) performs first
  and asks after (`r704_5j`); one that reacts to a change (`Changed<..>`) needs nothing.
- While any choice but `Target` is open nobody has priority (`TurnState::suspend_priority`): a fact about
  the holder, so no rule tests it. `Target` belongs to casting, the caster keeps priority.
- In tests the choice is answered where it opens, with its verb (`discard`, `arrange_graveyard`,
  `order_triggers`, `keep_legendary`, `assign_combat_damage`, `choose_defending_player`).

Rules that read a `PlayerAction` (play, cast, attack) keep doing so; the user parked moving them
to events, so don't propose it: such a rule checks, writes its own
`Rejected`, and then `Attempt(..)`, one event per declared object. Another rule performs it and writes `Did`
(`r508_1a` / `r508_1k`, `r509_1a` / `r509_1g` / `r509_1h`).

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

