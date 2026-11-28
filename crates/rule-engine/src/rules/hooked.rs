use crate::players;
use crate::rules;
use crate::rules::traits::hook_event::HookEvent;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hooked<E: HookEvent> {
    pub hook: E,
    pub player: players::PlayerId,
}

#[rustfmt::skip]
impl<E: HookEvent> rules::traits::event_args::EventArgs for Hooked<E> {
    fn arg(&self, id: rules::ArgId) -> Option<rules::ArgValue> { self.arg_value(id) }
    fn args() -> &'static [rules::ArgId] { &[Self::CARD, Self::PLAYER] }
}

#[rustfmt::skip]
impl<E: HookEvent> Hooked<E> {
    pub const CARD: rules::ArgId = rules::ArgId("card");
    pub const PLAYER: rules::ArgId = rules::ArgId("player");
}

#[rustfmt::skip]
impl<E: HookEvent> rules::traits::event::Event for Hooked<E> {
    const ID: rules::EventId = E::ID;
    fn restrictions() -> Vec<rules::Restriction> { E::restrictions() }
}

impl<E: HookEvent> Hooked<E> {
    pub(crate) fn arg_value(&self, id: rules::ArgId) -> Option<rules::ArgValue> {
        if id == Self::CARD {
            Some(rules::ArgValue::from(self.hook.card()))
        } else if id == Self::PLAYER {
            Some(rules::ArgValue::Player(self.player))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::cards;
    use crate::events;
    use crate::players;
    use crate::rules;
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
        assert_eq!(
            hooked.arg(Hooked::<events::OpeningHandAction>::CARD),
            Some(rules::ArgValue::Entity(*card))
        );
        assert_eq!(
            hooked.arg(Hooked::<events::OpeningHandAction>::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(
            hooked.arg(Hooked::<events::OpeningHandAction>::CARD),
            Some(rules::ArgValue::Entity(*card))
        );
        assert_eq!(
            hooked.arg(Hooked::<events::OpeningHandAction>::PLAYER),
            Some(rules::ArgValue::Player(player))
        );
        assert_eq!(
            Hooked::<events::OpeningHandAction>::args(),
            &[
                Hooked::<events::OpeningHandAction>::CARD,
                Hooked::<events::OpeningHandAction>::PLAYER,
            ]
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
