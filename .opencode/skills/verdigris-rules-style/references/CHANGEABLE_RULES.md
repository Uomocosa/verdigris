# Changeable rules

What cards can change and how. How a rule body is written: `../SKILL.md`.

## 0. The idea in one paragraph

In Magic every rule can be changed by a card: drawing, casting, paying, attacking, losing the
game. So the engine has ONE shape for all of them. Every game action is an **Event**. Before it
happens it is announced as `Attempt<E>` and passes through a fixed **pipeline**:

```
Attempt<E> -> 1. Permission (may E happen?  restrictions + AsThough / Allow / OnlyIf / Cant)
          -> 2. Replacement (614: does E become other events? Instead / Modify)
Resolve<E>                  (nothing can stop E any more; only the pipeline writes it, plus the
                           unstoppable rules listed in ACTING.md §5)
          -> 3. Perform     (the CR's default result, in the __rules__ system that owns E,
                           which reads Resolve<E>; an event whose CR default is
                           "nothing" has no such system)
          -> Did<E>         (triggers: "whenever a player draws ...")    (written by the rule that performs E)
```

A moment where, with no card reacting, NOTHING happens is not an event but a **hook**. The rule
announces a `HookEvent` list into `HookQueue<E>`; `hook_cards` applies every `Change::Hook` of each
card, one player at a time in the announced order; `HooksDone<E>` says it is over.

```
rule             -> hooks.announce(events_in_turn_order) into HookQueue<E>
hook_cards<E>    -> every Change::Hook of each card, one player at a time, in the announced order
HooksDone<E>     -> the queue is empty and no may is pending or accepted: the hook is over
```

Test: with no cards reacting, does anything happen? No: hook. Yes: `Attempt`. It reacts to something
that happened: `Did`.

A card never adds engine code to change a rule. It adds a `RuleChange { target, change }`
(a `Modification::Rule`) through `statics::all(...)`, and the pipeline reads it.

Quantities (402.2 hand size, 305.2 land plays) are not events; they are `Target::Value` and are
folded in timestamp order from the rule's default.

## 1. Vocabulary (the types; names are decided)

```rust
// rules/__traits__/event.rs : an Event IS its own context (subject / other / player).
pub trait Event: Context + Clone + Send + Sync + 'static {
    const ID: rules::EventId;                       // the CR rule number, e.g. "121.1"
    fn restrictions() -> Vec<rules::Restriction> { Vec::new() } // CR conditions; most events have none
}

// rules/__traits__/hook_event.rs : a hook IS its subject, the card that may act.
pub trait HookEvent: Clone + Send + Sync + 'static {
    const ID: rules::EventId;                        // the CR rule number, e.g. "103.6"
    fn card(&self) -> cards::CardId;                 // the card whose hook this is
    #[must_use]
    fn restrictions() -> Vec<rules::Restriction> { Vec::new() } // CR conditions; most hooks have none
}

// rules/__traits__/value_rule.rs (unchanged idea)
pub trait ValueRule: 'static { const ID: rules::RuleId; fn default() -> rules::Limit; }

pub enum Target {
    Event(rules::EventId),   // the whole action: "can't lose the game", "instead of drawing"
    Restriction(rules::RestrictionId),// one CR condition: haste, flash, "play lands from graveyard"
    Value(rules::RuleId),    // a quantity: hand size, land plays
}

pub enum Change {
    // permission (101.2: can't beats can; absolute restrictions can't be waived)
    AsThough,                                   // Restriction: treat as satisfied
    Allow(rules::Predicate),                    // Restriction: satisfied if this holds instead
    OnlyIf { role: rules::Role, condition: rules::Predicate }, // Event: veto unless it holds
    Cant { role: rules::Role },                   // Event: absolute veto for that role
    Must,                                       // Event: requirement (508.1d / 509.1c)
    Skip,                                       // Event: do not perform (untap, tap-attacker)
    Perform,                                    // Event: do perform (overrides Skip)
    // quantity
    Set(u32), Increase(u32), Decrease(u32), NoMaximum,
    // replacement (614)
    Instead { when: rules::Predicate, then: rules::Then }, // Then::{Events(Vec<EventTemplate>), May(EventTemplate)}
    // hook: ALL hooks of the subject apply (103.6: "some cards allow ...")
    Hook { when: rules::Predicate, then: rules::Then },
}

pub struct RuleChange { pub target: Target, pub change: Change }
```

- `EventTemplate` describes a replacement event in data; `Amount::{Fixed(u32), FromEvent(Field)}`
  lets it copy a parameter of the replaced event ("that much life", "that many cards").
- Predicates may read event parameters through `Field` (e.g. `Field::Amount`).

## 2. Nonsense is allowed, dangling is not

The user decided: maximum flexibility. Any `Target` may be paired with any `Change`; a pair
that means nothing (a `Set(3)` aimed at a restriction) is a silent no-op, not a type error. Do NOT
add typed wrappers that forbid pairs.

But a target must EXIST: every `EventId`, `RestrictionId` and value `RuleId` a card names must be
registered. A registry test walks every fixture card and fails on a dangling target. When you
add an event, restriction or value rule, register it.

## 3. How to add things

**A new game action** (draw, cast, pay mana, attack, mill ...):
1. A struct in `src/events/<cr_name>.rs` with the CR's words as its name and its parameters as
   fields (`Draw { player, amount }`). `impl Event` with `ID` = CR number, `Context` for roles.
2. If the CR lists conditions, write them as `restrictions()` with `RestrictionId` consts, text quoted
   VERBATIM from `docs/MagicCompRules-*.txt` (keep the verbatim test).
   A condition on the event's own numbers is a restriction too: `rules::event_field(rules::Field::Amount, ..)`
   (120.8 `events::deal_damage::NONZERO_DAMAGE`). The same `if event.amount == 0` in several rules is
   such a restriction.
3. The `__rules__` system that owns the action writes `Attempt<E>` instead of acting directly; the pipeline decides; the owning system reads `Resolve<E>` and performs only what comes out.
4. Register the event (section 2).

**A card that changes a rule**: pick the target (whole event, one restriction, or a value) and the
change; if the target event is missing, add it (above). Add one `statics::all(...)` builder in `src/statics/all.rs` + `__methods__/all/` if no
existing builder fits. Prefer reusing `as_though`, `allow`, `restriction`, `instead`.

**Replacement order** (616): earliest timestamp first; a replacement applies at most once to an
event (track applied effects on the event while it is re-announced). Player choice among
replacements (616.1) is NOT implemented yet. An optional "you may ... instead" belongs to
the change: `then: may(event)`. The pipeline asks through the shared ask / answer (`ACTING.md` §12)
and announces the event only on a yes. Never an implicit default, never a question in the rule.

```rust
// "If you would draw a card, you may mill a card instead." The rule that announces Draw is untouched.
Change::Instead { when: ALWAYS, then: may(EventTemplate::Mill { .. }) }
// the same without a choice: "If you would draw a card, mill a card instead."
Change::Instead { when: ALWAYS, then: Then::Events(vec![EventTemplate::Mill { .. }]) }
```

```
Attempt<Draw> -> pipeline finds the change on the event's subject / player
   Then::Events(vec![..])  -> announces Attempt<Mill>
   then: may(..)   -> May<Draw> (asks the event's player); yes -> Attempt<Mill>, no -> Resolve<Draw>
   no change       -> Resolve<Draw> (nothing reads it if the default is nothing)
```

The pipeline does the asking. A rule that reads `Resolve<E>` to build an ask is wrong: `may` already
asked, and the chosen event reaches the rule that owns it as its own `Resolve`.

Several optional changes of one player may be grouped into one ask. `then` may name any event: a new
template is one variant plus its `instantiate`, and the event's own rule performs it.

**"You don't ..." / "players don't ..."** (Upwelling: "Players don't lose unspent mana as steps and
phases end.") is `Change::Skip` on the event: `statics::all(PLAYER).dont_lose_unspent_mana()`. If the rule
performs the outcome itself, make it an event first.

**Text that points at cards** ("some cards / effects allow a player to ...") is a **hook**, not an
event: the rule announces a `HookEvent` for each object into `HookQueue<E>`, and the card writes a
change to that hook, for its own object — e.g.
`abilities::opening_hand_action(may(EventTemplate::BeginTheGameOnTheBattlefield))`. Nothing is asked
for an object without a change. An `Instead` aimed at a hook is a silent no-op (§2): `Rules::hooks`
reads only `Change::Hook`. At most one `may` hook per card and event (guard test in `rule_registry.rs`).
A hook on permanents needs a controller, which the engine doesn't have yet (`Hooked` uses
`cards::Owner`), so 502.1 phasing waits for it. "At the beginning of ..." is a triggered ability,
never a hook.

**Skipping a step**: a CR rule changes `turns::NextStep` (`TURNS.md` §17). No card skips a step
yet; the first one makes `NextStep` go through the pipeline.

**A card that works outside the battlefield** (611.3b "or the object generating it is in the
appropriate zone"): the change applies from the zone its text names. If the engine reads changes only
from the battlefield, that is a gap to close there, not a reason for a rule to look into the zone.

## 4. Do / don't

- Do look up changes with ONE helper (`Rules::changes(entity, target)`), never by matching
  `Modification` variants in each evaluator.
- Do look a change up on the entity its `Role` names (subject, other, player), for restrictions and
  for `OnlyIf` alike. `Cant` names the role it protects and vetoes only when the carrying
  entity is the event's entity of that role (exactly like `OnlyIf`). `Skip` / `Perform` fold
  in timestamp order over the subject's changes for the event (latest wins, unlike `Cant`).
- Don't create an empty "rule" type only to hang a veto on: the event is the target.
- Don't special-case a card in a `__rules__` system; express it as a `RuleChange`.
- Keep the CR numbers as IDs and the verbatim-text tests.
