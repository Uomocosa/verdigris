use crate::abilities;
use crate::cards;
use crate::effects;
use crate::rules;
use crate::rules::traits::event::Event;

#[must_use]
pub fn whenever(
    trigger: impl Into<abilities::Trigger>,
    effect: effects::Effect,
) -> abilities::TriggeredAbility {
    let trigger = trigger.into();
    abilities::TriggeredAbility {
        event: trigger.event,
        intervening_if: trigger.intervening_if,
        effect,
    }
}

#[must_use]
pub fn at(
    trigger: impl Into<abilities::Trigger>,
    effect: effects::Effect,
) -> abilities::TriggeredAbility {
    whenever(trigger, effect)
}

#[must_use]
pub fn when(
    trigger: impl Into<abilities::Trigger>,
    effect: effects::Effect,
) -> abilities::TriggeredAbility {
    whenever(trigger, effect)
}

#[must_use]
pub const fn when_this_dies() -> abilities::TriggerEvent {
    abilities::TriggerEvent::Dies
}

#[must_use]
pub const fn tap_add(produces: cards::ManaType) -> abilities::ActivatedAbility {
    abilities::ActivatedAbility {
        cost: abilities::ActivationCost::Tap,
        produces,
    }
}

#[must_use]
pub const fn begin_the_game_with_it_on_the_battlefield() -> abilities::StaticAbility {
    abilities::StaticAbility::Replacement(abilities::Replacement::Rule(rules::RuleChange {
        target: rules::Target::Event(rules::OpeningHandAction::ID),
        change: rules::Change::Instead {
            when: rules::Predicate::True,
            then: rules::may(rules::EventTemplate::BeginTheGameOnTheBattlefield),
        },
    }))
}
