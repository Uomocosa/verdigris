# Changeable rules

What cards can change and how. How a rule body is written: `../SKILL.md`.

## 0. The idea in one paragraph

In Magic every rule can be changed by a card: drawing, casting, paying, attacking, losing the
game. So the engine has ONE shape for all of them. Every game action is an **Event**. Before it
happens it is announced as `Attempt<E>` and passes through a fixed **pipeline**:

```
Attempt<E> -> 1. Permission (may E happen?  restrictions + AsThough / Allow / Restrict)
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
// rules/__traits__/event.rs : an Event IS its named arguments.
pub trait Event: EventArgs + Clone + Send + Sync + 'static {
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
    Event(rules::EventId),          // the whole action: "can't lose the game", "instead of drawing"
    Result(rules::ResultId),        // one named result of an event: 120.3a LOSES_LIFE
    Restriction(rules::RestrictionId), // one CR condition: haste, flash, "play lands from graveyard"
    Definition(rules::DefinitionId),   // one defined term: LETHAL_DAMAGE
    Value(rules::RuleId),           // a quantity: hand size, land plays
}

pub enum Change {
    // permission (101.2: can't beats can; unwaivable restrictions can't be waived)
    AsThough { holder: rules::ArgRef },                  // Restriction: treat as satisfied
    Allow { holder: rules::ArgRef, when: rules::Condition }, // Restriction/Definition: satisfied if this holds instead
    Restrict { holder: rules::ArgRef, restriction: rules::Restriction }, // Event: named veto for that holder
    Must,                                       // Event: requirement (508.1d / 509.1c)
    Skip { when: rules::Condition },            // Event: do not perform when `when` holds (vigilance: a Tap CausedBy attacking)
    Perform { when: rules::Condition },         // Event: do perform when `when` holds (overrides Skip)
    // quantity
    Set(u32), Increase(u32), Decrease(u32), NoMaximum,
    // replacement (614): one named result becomes other events
    Instead { holder: rules::ArgRef, when: rules::Condition, then: rules::Then }, // Then::{Events(Vec<EventTemplate>), May(EventTemplate)}
    // hook: an additional result alongside the default (lifelink)
    Hook { holder: rules::ArgRef, when: rules::Condition, then: rules::Then },
}

pub struct RuleChange { pub target: Target, pub change: Change }

// statements: one verbatim CR sentence; ids cite the rule ("509.1a/untapped", "702.9b/blockers")
pub type Restriction = rules::Statement<rules::RestrictionId>;
pub type Definition = rules::Statement<rules::DefinitionId>;
```

- `EventTemplate` describes a replacement event in data; `Amount::{Fixed(u32), FromArg(ArgId)}`
  lets it copy a parameter of the replaced event ("that much life", "that many cards").
- Conditions may read event parameters through `ArgId` (e.g. `DealDamage::AMOUNT.is(rules::Value(1).or_more())`).
- A list argument (`ArgValue::Entities`, e.g. `DeclareAttackers::ATTACKERS`) is a holder and a subject element by
  element: a change held by one element is looked up on that element, and `.of(LIST)` / `points_to` / `in_zone` /
  `X.controller()` hold only if they hold for EVERY element (`Is`: the entity is one of them). `count(LIST)` reads its size.
- `Condition::CausedBy(id)` is true when the event's `cause` argument is `id` (§6.4).

## 2. Nonsense is allowed, dangling is not

The user decided: maximum flexibility. Any `Target` may be paired with any `Change`; a pair
that means nothing (a `Set(3)` aimed at a restriction) is a silent no-op, not a type error. Do NOT
add typed wrappers that forbid pairs.

But a target must EXIST: every `EventId`, `RestrictionId` and value `RuleId` a card names must be
registered. A registry test walks every fixture card and fails on a dangling target. When you
add an event, restriction or value rule, register it.

## 3. How to add things

**A new game action** (draw, cast, pay mana, attack, mill ...):
1. A struct in `__events__/<cr_name>.rs` with the CR's words as its name and its parameters as
   fields (`Draw { player, amount }`). `impl Event` with `ID` = CR number, named `#[arg]`s.
2. If the CR lists conditions, write them as `restrictions()` with `RestrictionId` consts
   (the id cites the rule, e.g. `RestrictionId("508.1a/controls")`; the doc comment on the
   rule that performs the event quotes the sentence). No text is stored on the restriction.
   A condition on the event's own numbers is a restriction too: `DealDamage::AMOUNT.is(..)`
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
Change::Instead { holder: events::Draw::PLAYER.into(), when: rules::Condition::True, then: may(EventTemplate::Mill { .. }) }
// the same without a choice: "If you would draw a card, mill a card instead."
Change::Instead { holder: events::Draw::PLAYER.into(), when: rules::Condition::True, then: Then::Events(vec![EventTemplate::Mill { .. }]) }
```

```
Attempt<Draw> -> pipeline finds the change on the event's entities
   Then::Events(vec![..])  -> announces Attempt<Mill>
   then: may(..)   -> May<Draw> (asks the event's player); yes -> Attempt<Mill>, no -> Resolve<Draw>
   no change       -> Resolve<Draw> (nothing reads it if the default is nothing)
```

The pipeline does the asking. A rule that reads `Resolve<E>` to build an ask is wrong: `may` already
asked, and the chosen event reaches the rule that owns it as its own `Resolve`.

Several optional changes of one player may be grouped into one ask. `then` may name any event: a new
template is one variant plus its `instantiate`, and the event's own rule performs it.

**"You don't ..." / "players don't ..."** (Upwelling: "Players don't lose unspent mana as steps and
phases end.") is `Change::Skip { when: rules::Condition::True }` on the event: `statics::all(PLAYER).dont_lose_unspent_mana()`. If the rule
performs the outcome itself, make it an event first.

**Named arguments.** An event derives `EventArgs` (`#[derive(EventArgs)]`, `crates/event-derive`),
marking each named argument with `#[arg]`; the derive emits one `SCREAMING_SNAKE_CASE` const per field
(`DealDamage::SOURCE`) plus `EventArgs::arg` (a value by id) and `EventArgs::args` (all ids). The pipeline
reads only the named arguments; there is no positional `subject` / `other` / `player`.

**Text that points at cards** ("some cards / effects allow a player to ...") is a **hook**, not an
event: the rule announces a `HookEvent` for each object into `HookQueue<E>`, and the card writes a
change to that hook, for its own object — e.g.
`abilities::opening_hand_action(may(EventTemplate::BeginTheGameOnTheBattlefield))`. Nothing is asked
for an object without a change. An `Instead` aimed at a hook is a silent no-op (§2): `Rules::hooks`
reads only `Change::Hook`. At most one `may` hook per card and event (guard test in `rule_registry.rs`).
A hook names its player explicitly (`Hooked<E> { hook, player }`, `src/rules/hooked.rs`). "At the
beginning of ..." is a triggered ability, never a hook.

**Skipping a step**: a CR rule changes `turns::NextStep` (`TURNS.md` §17). No card skips a step
yet; the first one makes `NextStep` go through the pipeline.

**A card that works outside the battlefield** (611.3b "or the object generating it is in the
appropriate zone"): the change applies from the zone its text names. If the engine reads changes only
from the battlefield, that is a gap to close there, not a reason for a rule to look into the zone.

## 4. Do / don't

- Do look up changes with ONE helper (`Rules::changes(entity, target)`), never by matching
  `Modification` variants in each evaluator.
- Do look a change up on the entity its `holder` names (each named argument), for waivers
  of restrictions and for `Restrict` alike. `Restrict` names the argument it protects in `holder` and vetoes only when the
  carrying entity is the event's entity of that argument. `Skip` / `Perform` fold
  in timestamp order over the changes on the event's entities for the event (latest wins, unlike `Restrict`).
- Don't create an empty "rule" type only to hang a veto on: the event is the target.
- Don't special-case a card in a `__rules__` system; express it as a `RuleChange`.
- Keep the CR numbers as IDs and the rule-number tests.

## 5. Writing a keyword or a rule change

One section, one shape. Every example below compiles against the current code; paths are relative
to `crates/rule-engine/`.

**An event names its arguments.** `#[derive(EventArgs)]` (`crates/event-derive`) with `#[arg]` on
each field emits one `SCREAMING_SNAKE_CASE` const per field plus `EventArgs::arg` / `args`; the
pipeline reads only named arguments. No derived controllers or owners are stored on the event
(`__events__/deal_damage.rs`):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct DealDamage {
    #[arg]
    pub source: Entity,
    #[arg]
    pub recipient: Entity,
    #[arg]
    pub amount: u32,
}
```

**`X.controller()` / `X.owner()` are derived, never stored.** `ArgId::controller()` and
`ArgId::owner()` build an `ArgRef` (`src/rules/arg_ref.rs`); `Rules::resolve` derives the player
from the card's `Controller` / `Owner` component at rule time (`src/rules/rules.rs`):

```rust
answers.resolve(events::DealDamage::SOURCE.controller(), &damage) // Some(ArgValue::Player(p1.id))
```

**A restriction names whom it asks about.** Conditions say whom they ask about with `.of(...)`
(`src/rules/condition.rs`); amounts read an argument with `E::AMOUNT.is(...)` etc.
(`src/rules/arg_id.rs`). From `__events__/declare_blocker.rs`:

```rust
rules::has::<cards::Creature>().of(DeclareBlocker::BLOCKER)
DealDamage::AMOUNT.is(rules::Value(1).or_more()) // the 120.8 nonzero restriction, events::deal_damage::NONZERO_DAMAGE
```

**Comparisons.** A quantity becomes a `rules::Condition` with `.is(...)`
(`crates/rule-engine/src/rules/player_quantity.rs:16`, `player_quantity_of.rs:12`,
`arg_id.rs:17`, `count.rs:11`, `characteristic.rs:12`). The comparison is built from a
value or a threshold (`src/rules/value.rs:10`, `src/rules/__basic__/condition_builders.rs:35`):

```rust
rules::life().of(YOU).is(rules::Value(40).or_more())
rules::counters(counters::POISON).is(rules::Value(0).or_less())
rules::power().of(THIS).is(rules::Value(3).or_more())
rules::toughness().is(rules::Value(2).or_less())
rules::cards_in_hand().is(rules::exactly(0))
rules::cards_in_library().is(rules::greater_than(5))
rules::life().is(rules::less_than(10))
events::AssignCombatDamage::AMOUNT.is(rules::greater_than(0))
rules::count(events::DeclareAttackers::ATTACKERS).is(rules::Value(1).or_less())
```

`YOU` is the controller of the spell or ability being evaluated
(`src/rules/arg_ref.rs:11`: `events::AbilityContext::SPELL_OR_ABILITY.controller()`), so
`rules::life().of(YOU)` always asks about that object's controller
(`src/rules/player_quantity_of.rs:24`).

`THIS` is the spell or ability's source
(`rules::ArgRef::SourceOf(events::AbilityContext::SPELL_OR_ABILITY)`), so
`rules::power().of(THIS).is(rules::Value(3).or_more())` asks about the source's own
power — for a dies trigger, its last-known power (CR 603.10). Test Last Stand
(`src/testing/__fixtures__/cards/test_last_stand.rs`) reads "when this dies, if its
power is 3 or more, draw a card."

**Quantifiers.** `rules::each` / `rules::any` ask about several players at once
(`src/rules/__basic__/condition_builders.rs:120`):

```rust
rules::each(OPPONENT, rules::life().is(rules::Value(10).or_less()))
rules::any(OPPONENT, rules::life().is(rules::Value(10).or_less()))
rules::any(PLAYER, rules::life().is(rules::Value(40).or_more()))
```

**Static "as long as".** A continuous ability holds only while its condition holds
(`src/abilities/continuous.rs:10`); the layers re-check it against
`events::AbilityContext { spell_or_ability }` (`__rules__/r611_3a_static_effects.rs:190`). Serra
Ascendant (`src/testing/__fixtures__/cards/serra_ascendant.rs:16`):

```rust
statics::this().get_pt(5, 5).as_long_as(rules::life().of(YOU).is(rules::Value(30).or_more()))
statics::this().gain(keywords::flying()).as_long_as(rules::life().of(YOU).is(rules::Value(30).or_more()))
```

**Intervening "if".** A trigger written `Event.if_(<rules::Condition>)`
(`src/abilities/trigger_event.rs:51`) stores `intervening_if` and is checked twice: at
trigger time (603.4, `__rules__/r603_4_intervening_if.rs:66`) and on resolution (608.2a,
`__rules__/r608_2a_intervening_if.rs:27`), both against
`events::AbilityContext { spell_or_ability }` (`__events__/ability_context.rs:8`). Felidar Sovereign
(`src/testing/__fixtures__/cards/felidar_sovereign.rs:22`):

```rust
turns::beginning_of(UPKEEP).of(YOU)
    .if_(rules::life().of(YOU).is(rules::Value(40).or_more()))
```

**A restriction vetoes one holder; a waiver names the same holder.** `Change::Restrict { holder,
restriction }` vetoes only when the carrying entity plays the `holder` argument; `AsThough {
holder }` / `Allow { holder, when }` waive it for that holder. Holders are `ArgRef`s built from the
event's own consts. "Can't" beats "can" (101.2); permissions are OR'd, so restating one is harmless.
Flying (`__keywords__/r702_9b_flying.rs`) restricts the attacker; reach
(`__keywords__/r702_17b_reach.rs`) waives the same restriction for the blocker:

```rust
// flying: "can't be blocked except by creatures with flying and/or reach" (702.9b)
rules::Change::Restrict {
    holder: events::DeclareBlocker::ATTACKER.into(),
    restriction: BLOCKERS.restriction(
        (rules::has_keyword(keywords::flying::ID) | rules::has_keyword(keywords::reach::ID))
            .of(events::DeclareBlocker::BLOCKER),
    ),
}
// reach: waives flying's restriction (702.17b repeats 702.9b verbatim)
rules::Change::AsThough { holder: events::DeclareBlocker::BLOCKER.into() }
// defender: "can't attack" (702.3b)
rules::Change::Restrict {
    holder: events::DeclareAttackers::ATTACKERS.into(),
    restriction: CANT_ATTACK.restriction(!rules::Condition::True),
}
```

**A named result is replaced with `Instead`, extended with `Hook`.** `ResultId` names one result of
an event (`LOSES_LIFE`, `src/constants.rs`). The performing rule asks `Rules::replaced` and announces
the winner's `Then` instead of its default (`__rules__/r120_3a_damage_to_a_player.rs`). Infect
(`__keywords__/r702_90b_infect.rs`) replaces the result; lifelink
(`__keywords__/r702_15b_lifelink.rs`) adds one with `Hook`:

```rust
// infect (702.90b) replaces 120.3a LOSES_LIFE with poison counters
rules::Change::Instead {
    holder: events::DealDamage::SOURCE.into(),
    when: rules::Condition::True,
    then: rules::Then::Events(vec![rules::EventTemplate::PutCounters {
        who: events::DealDamage::RECIPIENT.into(),
        kind: counters::POISON,
        amount: rules::Amount::FromArg(events::DealDamage::AMOUNT),
    }]),
}
// lifelink (702.15b) adds "that much life" alongside the damage
rules::Change::Hook {
    holder: events::DealDamage::SOURCE.into(),
    when: rules::Condition::True,
    then: rules::Then::Events(vec![rules::EventTemplate::GainLife {
        who: events::DealDamage::SOURCE.controller(),
        amount: rules::Amount::FromArg(events::DealDamage::AMOUNT),
    }]),
}
```

**A definition is widened with `Allow`.** `DefinitionId` names a defined term (`LETHAL_DAMAGE`,
`src/rules/definition_id.rs`); a rule asks `Rules::defines` and the keyword widens it. Deathtouch
(`__keywords__/r702_2c_deathtouch.rs`): any nonzero amount counts as lethal:

```rust
rules::RuleChange {
    target: rules::Target::Definition(rules::LETHAL_DAMAGE),
    change: rules::Change::Allow {
        holder: events::AssignCombatDamage::SOURCE.into(),
        when: events::AssignCombatDamage::AMOUNT.is(rules::greater_than(0)),
    },
}
```

**A keyword = marker (id) + abilities, one file per keyword.** `__keywords__/r702_9b_flying.rs`: the
`pub fn` is named after the file stem, `pub const ID` is the marker, owned restriction ids
(`BLOCKERS`) live next to it, and the private `abilities()` returns the `RuleChange`s. Plain
keywords use `Keyword::from_plain(ID, abilities)`; parameterised ones use `Keyword::from_params`
(`src/keywords/keyword.rs`). Card text calls the alias (`keywords::flying()`, re-exported from
`src/keywords/__basic__/card_text.rs`); the file's own consts are reached as
`keywords::flying::BLOCKERS` (`src/keywords/mod.rs` re-exports each file `pub(crate)`).

The doc comment quotes the CR rule verbatim, wrapped at 100 columns (VRDG015), one paragraph per
rule. `# Restates` lists CR rules elsewhere that restate the keyword (702.17b for reach, 120.3b for
infect, 120.3f for lifelink); `# Reminder text` quotes the card reminder text ("This creature can
block creatures with flying."). Rules a keyword only conditions or repeats get no `__rules__` file:
their text lives under the keyword's `# Restates`, and a `__rules__` file that performed them is
deleted (VRDG014).

**Literal mapping.** Each keyword maps ITS OWN text; no shortcut folds one keyword's text into
another (reach does not grant "can block flyers" — it waives flying's restriction, because 702.17b
says what 702.9b says). An EFFECT restated by the CR in two places is implemented once — in the
keyword file (decided: option A): 120.3b lives under infect's `# Restates`, 120.3f under lifelink's,
and `r702_15b` / `r120_3b` rule files were deleted.

## 6. How an event is shaped

Decided by the user (2026-11-27). Every event struct passes all five; the Smells table in `SKILL.md` points here.

### 6.1 No field readable from another field

If a value can be read from an entity already in the event (controller, owner, source, targets), it is not a field:
ask `X.controller()` / `X.owner()` / `SourceOf(X)`. If the entity does not carry it yet, the rule that makes the event
sets it first, as the CR does.

```rust
// Good: 120.2, the damage is dealt by the source's controller (__events__/deal_damage.rs)
pub struct DealDamage { #[arg] pub source: Entity, #[arg] pub recipient: Entity, #[arg] pub amount: u32 }
// who deals = DealDamage::SOURCE.controller()
// Bad
pub struct CastSpell { #[arg] pub spell: Entity, #[arg] pub player: players::PlayerId }
```

Exception: `Cast.caster` and `Play.player`. They name the player proposing the action, who becomes its
controller (601.2a / 305.1). They can't be read from the card before the action exists, so they are not
inferable fields.

**Why:** two fields that must agree will one day disagree; a derived one cannot.

### 6.2 General verb, specific filter

Name the event after the CR verb, not one case of it. The case is a filter on the arguments.

```rust
// Good
Play { card }                       // "whenever you play a land" = a Play trigger filtered by land()
PutCounters { recipient, kind, amount }
EnterTheBattlefield { permanent }   // the CR's words
// Bad
PlayLand { land }, GivePoisonCounters { player, amount }, EnterBattlefield { .. }
```

**Why:** one event per verb means one pipeline, one set of restrictions, and every card that cares about the verb sees it.

### 6.3 One plural event per verb

An action on several objects is ONE event with a list or an amount. A single case is the same event with one item;
doing it twice is two events.

```rust
// Good
DeclareAttackers { attackers: Vec<Entity> }   // vec![a] = "a attacks", vec![] = no attack
Discard { cards: Vec<Entity>, cause }         // player = the cards' owner
Draw { player, amount, cause }                // "draw one, then draw one" = two Draw { amount: 1 }
// Bad
DeclareAttacker { attacker } + DeclareAttackers { attackers }
Discard { player, amount } + DiscardCard { card }
```

A list argument applies to each element (§1); `count(..)` / `AMOUNT` reads the batch, which replacements need:
Painful Lessons (`src/testing/__fixtures__/cards/test_painful_lessons.rs`) "if a player would draw two or more cards".

**Why:** a singular and a plural event split restrictions and triggers between two places; the CR asks about the
whole set (508.1c "can't attack alone") and about each element, and one list argument answers both.

### 6.4 Every state change says why

A state-changing event has a required `#[arg] pub cause: rules::EventId`: the producing event's `ID`, or, for a
producer that is not an event (state-based actions, 608.2 resolution, a step), that rule's own
`pub const CAUSE: rules::EventId` (e.g. `"704.5g/lethal-damage"`, `"608.2/resolve"`). A card that cares reads it
with `Condition::CausedBy(..)`.

```rust
// Good: vigilance (702.20b) "attacking doesn't cause it to tap"
Change::Skip { when: rules::Condition::CausedBy(events::DeclareAttackers::ID) }   // target: Tap
// Bad: a wrapper event that only carries the reason
TapAttacker { attacker }  ->  Tap { permanent }
```

**Why:** "whenever a creature dies from lethal damage", "tapped for mana", "discarded by an opponent" are all
questions about the cause; a field answers them all, a wrapper event per reason does not scale.

### 6.5 Bookkeeping is not an event field

State that a rule already tracks lives in its component, not on the event.

```rust
// Good: everything that enters gets cards::SummoningSick; 302.6 lifts it at its controller's turn start
EnterTheBattlefield { permanent }
// Bad
EnterBattlefield { permanent, controller, summoning_sick: bool }
```

