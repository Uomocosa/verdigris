use crate::abilities;
use crate::cards;
use crate::effects;
use crate::events;
use crate::rules;

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
pub const fn tap_add(produces: cards::ManaType) -> abilities::ActivatedAbility {
    abilities::ActivatedAbility {
        cost: abilities::ActivationCost::Tap,
        produces,
    }
}

#[must_use]
pub fn opening_hand_action(then: impl Into<rules::Then>) -> abilities::StaticAbility {
    abilities::StaticAbility::Replacement(abilities::Replacement::Rule(rules::RuleChange {
        target: rules::Target::event::<rules::Hooked<events::OpeningHandAction>>(),
        change: rules::Change::Hook {
            holder: rules::Hooked::<events::OpeningHandAction>::CARD.into(),
            when: rules::always(),
            then: then.into(),
        },
    }))
}

#[must_use]
pub fn begin_the_game_with_it_on_the_battlefield() -> abilities::StaticAbility {
    opening_hand_action(rules::may(
        rules::EventTemplate::BeginTheGameOnTheBattlefield,
    ))
}
