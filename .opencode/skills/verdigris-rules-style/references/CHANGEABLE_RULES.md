# Changeable rules

What cards can change and how. How a rule body is written: `../SKILL.md`.

## 0. The idea in one paragraph

In Magic every rule can be changed by a card: drawing, casting, paying, attacking, losing the
game. So the engine has ONE shape for all of them. Every game action is an **Event**. Before it
happens it is announced as `Would<E>` and passes through a fixed **pipeline**:

```
Would<E> -> 1. Permission  (may E happen?  clauses + AsThough / Allow / OnlyIf / Cant)
         -> 2. Replacement (614: does E become other events? Instead / Modify)
Proceeds<E>                (nothing can stop E any more; only the pipeline writes it, plus the
                           unstoppable rules listed in ../SKILL.md §5)
         -> 3. Perform     (the CR's default result, in the __rules__ system that owns E,
                           which reads Proceeds<E>; an event whose CR default is
                           "nothing" has no such system)
         -> Did<E>         (triggers: "whenever a player draws ...")    [when needed]
```

A card never adds engine code to change a rule. It adds a `RuleChange { target, change }`
(a `Modification::Rule`) through `statics::all(...)`, and the pipeline reads it.

Quantities (402.2 hand size, 305.2 land plays) are not events; they are `Target::Value` and are
folded in timestamp order from the rule's default.

## 1. Vocabulary (the types; names are decided)

```rust
// rules/__traits__/event.rs : an Event IS its own context (subject / other / player).
pub trait Event: Context + Clone + Send + Sync + 'static {
    const ID: rules::EventId;                       // the CR rule number, e.g. "121.1"
    fn clauses() -> Vec<rules::Clause> { Vec::new() } // CR conditions; most events have none
}

// rules/__traits__/value_rule.rs (unchanged idea)
pub trait ValueRule: 'static { const ID: rules::RuleId; fn default() -> rules::Limit; }

pub enum Target {
    Event(rules::EventId),   // the whole action: "can't lose the game", "instead of drawing"
    Clause(rules::ClauseKey),// one CR condition: haste, flash, "play lands from graveyard"
    Value(rules::RuleId),    // a quantity: hand size, land plays
}

pub enum Change {
    // permission (101.2: can't beats can; absolute clauses can't be waived)
    AsThough,                                   // Clause: treat as satisfied
    Allow(rules::Predicate),                    // Clause: satisfied if this holds instead
    OnlyIf { role: rules::Role, condition: rules::Predicate }, // Event: veto unless it holds
    Cant { role: rules::Role },                   // Event: absolute veto for that role
    Must,                                       // Event: requirement (508.1d / 509.1c)
    Skip,                                       // Event: do not perform (untap, tap-attacker)
    Perform,                                    // Event: do perform (overrides Skip)
    // quantity
    Set(u32), Increase(u32), Decrease(u32), NoMaximum,
    // replacement (614)
    Instead { when: rules::Predicate, then: rules::Then }, // Then::{Events(Vec<EventTemplate>), May(EventTemplate)}
}

pub struct RuleChange { pub target: Target, pub change: Change }
```

- `EventTemplate` describes a replacement event in data; `Amount::{Fixed(u32), FromEvent(Field)}`
  lets it copy a parameter of the replaced event ("that much life", "that many cards").
- Predicates may read event parameters through `Field` (e.g. `Field::Amount`).

## 2. Nonsense is allowed, dangling is not

The user decided: maximum flexibility. Any `Target` may be paired with any `Change`; a pair
that means nothing (a `Set(3)` aimed at a clause) is a silent no-op, not a type error. Do NOT
add typed wrappers that forbid pairs.

But a target must EXIST: every `EventId`, `ClauseKey` and value `RuleId` a card names must be
registered. A registry test walks every fixture card and fails on a dangling target. When you
add an event, clause or value rule, register it.

## 3. How to add things

**A new game action** (draw, cast, pay mana, attack, mill ...):
1. A struct in `src/rules/<cr_name>.rs` with the CR's words as its name and its parameters as
   fields (`Draw { player, amount }`). `impl Event` with `ID` = CR number, `Context` for roles.
2. If the CR lists conditions, write them as `clauses()` with `ClauseId` consts, text quoted
   VERBATIM from `docs/MagicCompRules-*.txt` (keep the verbatim test).
3. The `__rules__` system that owns the action writes `Would<E>` instead of acting directly; the pipeline decides; the owning system reads `Proceeds<E>` and performs only what comes out.
4. Register the event (section 2).

**A card that changes a rule**: pick the target (whole event, one clause, or a value) and the
change; if the target event is missing, add it (above). Add one `statics::all(...)` builder in `src/statics/all.rs` + `__methods__/all/` if no
existing builder fits. Prefer reusing `as_though`, `allow`, `restriction`, `instead`.

**Replacement order** (616): earliest timestamp first; a replacement applies at most once to an
event (track applied effects on the event while it is re-announced). Player choice among
replacements (616.1) is NOT implemented yet. An optional "you may ... instead" belongs to
the change: `then: may(event)`. The pipeline asks through the shared ask / answer (`../SKILL.md` §12)
and announces the event only on a yes. Never an implicit default, never a question in the rule.

```rust
// "If you would draw a card, you may mill a card instead." The rule that announces Draw is untouched.
Change::Instead { when: ALWAYS, then: may(EventTemplate::Mill { .. }) }
// the same without a choice: "If you would draw a card, mill a card instead."
Change::Instead { when: ALWAYS, then: Then::Events(vec![EventTemplate::Mill { .. }]) }
```

```
Would<Draw> -> pipeline finds the change on the event's subject / player
   Then::Events(vec![..])  -> announces Would<Mill>
   then: may(..)   -> Ask(the event's player); yes -> Would<Mill>, no -> Proceeds<Draw>
   no change       -> Proceeds<Draw> (nothing reads it if the default is nothing)
```

The pipeline does the asking. A rule that reads `Proceeds<E>` to build an ask is wrong: `may` already
asked, and the chosen event reaches the rule that owns it as its own `Proceeds`.

Several optional changes of one player may be grouped into one ask. `then` may name any event: a new
template is one variant plus its `instantiate`, and the event's own rule performs it.

**Text that points at cards** ("some cards / effects allow a player to ...") is announced as an event
whose subject is the object that may carry the change; its default is nothing. The card then writes a
change to that event, for its own object. Nothing is asked for an object without a change.

**A card that works outside the battlefield** (611.3b "or the object generating it is in the
appropriate zone"): the change applies from the zone its text names. If the engine reads changes only
from the battlefield, that is a gap to close there, not a reason for a rule to look into the zone.

## 4. Do / don't

- Do look up changes with ONE helper (`Rules::changes(entity, target)`), never by matching
  `Modification` variants in each evaluator.
- Do look a change up on the entity its `Role` names (subject, other, player), for clauses and
  for `OnlyIf` alike. `Cant` names the role it protects and vetoes only when the carrying
  entity is the event's entity of that role (exactly like `OnlyIf`). `Skip` / `Perform` fold
  in timestamp order over the subject's changes for the event (latest wins, unlike `Cant`).
- Don't create an empty "rule" type only to hang a veto on: the event is the target.
- Don't special-case a card in a `__rules__` system; express it as a `RuleChange`.
- Keep the CR numbers as IDs and the verbatim-text tests.
