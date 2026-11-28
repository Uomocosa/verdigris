use std::marker::PhantomData;

use atomic_delegate_macros::atomic_delegates;

use crate::abilities;
use crate::effects;
use crate::events;
use crate::rules;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Would<E: rules::traits::event::Event> {
    pub filter: effects::Filter,
    pub holder: rules::ArgRef,
    pub when: rules::Condition,
    pub event: PhantomData<E>,
}

impl<E: rules::traits::event::Event> Would<E> {
    #[must_use]
    pub fn of(filter: impl Into<effects::Filter>) -> Self {
        Self {
            filter: filter.into(),
            holder: E::HOLDER,
            when: rules::Condition::True,
            event: PhantomData,
        }
    }

    #[must_use]
    pub fn when(mut self, when: rules::Condition) -> Self {
        self.when = self.when & when;
        self
    }

    #[must_use]
    pub fn by(mut self, holder: impl Into<rules::ArgRef>) -> Self {
        self.holder = holder.into();
        self
    }
}

#[atomic_delegates]
impl<E: rules::traits::event::Event> Would<E> {
    #[must_use]
    pub fn instead(self, then: impl Into<rules::Then>) -> abilities::Continuous {}
}

impl Would<events::DealDamage> {
    #[must_use]
    pub fn to(mut self, who: impl Into<rules::Subject>) -> Self {
        self.when = self.when & rules::is(events::DealDamage::RECIPIENT).of(who);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::OPPONENT;
    use crate::creature;
    use crate::effects;
    use crate::events;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::statics;

    #[test]
    fn test_usage() {
        let pending: statics::Would<events::Draw> = statics::Would::of(creature());
        assert_eq!(pending.filter, creature());
        assert_eq!(pending.holder, rules::ArgRef::Arg(events::Draw::PLAYER));
        assert_eq!(pending.when, rules::Condition::True);
        let narrowed = pending.when(rules::Condition::True);
        assert_eq!(
            narrowed.when,
            rules::Condition::True & rules::Condition::True
        );
    }

    #[test]
    fn by_overrides_the_event_holder() {
        assert_eq!(
            events::DealDamage::HOLDER,
            rules::ArgRef::Arg(events::DealDamage::SOURCE)
        );
        let ability = statics::if_a(creature())
            .would::<events::DealDamage>()
            .by(events::DealDamage::RECIPIENT)
            .instead(vec![rules::EventTemplate::GainLife {
                who: events::DealDamage::RECIPIENT.into(),
                amount: rules::Amount::Fixed(1),
            }]);
        assert!(matches!(
            ability.modification,
            effects::Modification::Rule(rules::RuleChange {
                change: rules::Change::Instead { holder, .. },
                ..
            }) if holder == events::DealDamage::RECIPIENT.into()
        ));
    }

    #[test]
    fn to_adds_the_recipient_conjunct() {
        let pending = statics::if_a(creature()).would::<events::DealDamage>();
        assert_eq!(pending.when, rules::Condition::True);
        let aimed = pending.to(rules::an(OPPONENT));
        assert_eq!(
            aimed.when,
            rules::Condition::True
                & rules::is(events::DealDamage::RECIPIENT).of(rules::an(OPPONENT))
        );
    }
}
