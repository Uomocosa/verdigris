---
name: verdigris-changeable-rules
description: Use before adding or changing ANY game action, rule, card effect, static ability, replacement effect or rule-changing card in verdigris. Defines the "changeable rules" pattern - every game action is an Event that passes through Would<E> (permission, replacement, perform), and every card that changes a rule is one RuleChange { target, change } aimed at an event, a clause, or a value. Read together with verdigris-style.
---

# verdigris-changeable-rules

## 0. The idea in one paragraph

In Magic every rule can be changed by a card: drawing, casting, paying, attacking, losing the
game. So the engine has ONE shape for all of them. Every game action is an **Event**. Before it
happens it is announced as `Would<E>` and passes through a fixed **pipeline**:

```
Would<E> -> 1. Permission  (may E happen?  clauses + AsThough / Allow / OnlyIf / Cant)
         -> 2. Replacement (614: does E become other events? Instead / Modify)
         -> 3. Perform     (the CR's default result, in the __rules__ system that owns E)
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
    Instead { when: rules::Predicate, then: Vec<rules::EventTemplate> },
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
3. The `__rules__` system that owns the action writes `Would<E>` instead of acting directly; the
   pipeline decides; the owning system performs only what comes out.
4. Register the event (section 2).

**A card that changes a rule**: pick the target (whole event, one clause, or a value) and the
change; add one `statics::all(...)` builder in `src/statics/all.rs` + `__methods__/all/` if no
existing builder fits. Prefer reusing `as_though`, `allow`, `restriction`, `instead`.

**Replacement order** (616): earliest timestamp first; a replacement applies at most once to an
event (track applied effects on the event while it is re-announced). Player choice among
replacements (616.1) and optional "you may ... instead" are NOT implemented yet: add them as a
pending choice like the opening-hand choices, never as an implicit default.

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
