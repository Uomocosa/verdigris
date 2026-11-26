use bevy::prelude::Entity;

use crate::players;
use crate::rules;
use crate::rules::traits::hook_event::HookEvent;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hooked<E: HookEvent> {
    pub hook: E,
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl<E: HookEvent> rules::traits::context::Context for Hooked<E> {
    fn subject(&self) -> Entity { *self.hook.card() }
    fn other(&self) -> Option<Entity> { None }
    fn player(&self) -> players::PlayerId { self.player }
}

#[rustfmt::skip]
impl<E: HookEvent> rules::traits::event_args::EventArgs for Hooked<E> {
    fn arg(&self, id: rules::ArgId) -> Option<rules::ArgValue> {
        (id == Self::CARD).then_some(rules::ArgValue::from(self.hook.card()))
    }
    fn args() -> &'static [rules::ArgId] { &[Self::CARD] }
}

#[rustfmt::skip]
impl<E: HookEvent> Hooked<E> {
    pub const CARD: rules::ArgId = rules::ArgId("card");
}

#[rustfmt::skip]
impl<E: HookEvent> rules::traits::event::Event for Hooked<E> {
    const ID: rules::EventId = E::ID;
    fn restrictions() -> Vec<rules::Restriction> { E::restrictions() }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::cards;
    use crate::events;
    use crate::players;
    use crate::rules;
    use crate::rules::traits::context::Context;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;
    use crate::rules::traits::hook_event::HookEvent;

    use super::Hooked;

    #[test]
    fn test_usage() {
        let card = cards::CardId(Entity::PLACEHOLDER);
        let player = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let hooked = Hooked {
            hook: events::OpeningHandAction { card },
            player,
        };
        assert_eq!(hooked.subject(), *card);
        assert_eq!(hooked.other(), None);
        assert_eq!(hooked.player(), player);
        assert_eq!(
            hooked.arg(Hooked::<events::OpeningHandAction>::CARD),
            Some(rules::ArgValue::Entity(*card))
        );
        assert_eq!(
            Hooked::<events::OpeningHandAction>::args(),
            &[Hooked::<events::OpeningHandAction>::CARD]
        );
        assert_eq!(
            Hooked::<events::OpeningHandAction>::ID,
            rules::EventId("103.6/opening-hand-action")
        );
        assert_eq!(
            Hooked::<events::OpeningHandAction>::restrictions(),
            events::OpeningHandAction::restrictions()
        );
    }
}
