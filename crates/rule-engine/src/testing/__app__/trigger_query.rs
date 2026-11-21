use derive_more::Deref;

use crate::testing;

#[derive(Clone, Copy, Debug, Deref)]
pub struct TriggerQuery(pub(crate) testing::Card);

impl From<testing::Card> for TriggerQuery {
    fn from(card: testing::Card) -> Self {
        Self(card)
    }
}

impl TriggerQuery {
    pub(crate) fn matches(self, object: &testing::ZoneObject) -> Option<testing::TestTrigger> {
        match object {
            testing::ZoneObject::Trigger {
                ability,
                source_name,
            } if source_name.as_str() == *self.name => Some(*ability),
            testing::ZoneObject::Trigger { .. } | testing::ZoneObject::Card { .. } => None,
        }
    }

    pub(crate) fn label(self) -> String {
        let Self(card) = self;
        card.name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::HAND;
    use crate::mana;
    use crate::testing::*;

    #[test]
    fn test_usage() -> TestResult {
        let query = SOUL_WARDEN.trigger();
        assert_eq!(query.label(), "Soul Warden");
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([SOUL_WARDEN]).hand([GRIZZLY_BEARS]).mana_pool(mana![1, G]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([p1.cast(bears).pass_priority(), p2.pass_priority()])?;
        let stack = game.stack();
        assert!(stack.has_trigger(query));
        assert!(stack.has_no_trigger(MURDER.trigger()));
        let [trigger] = stack.exactly_trigger::<1>(query)?;
        assert_eq!(Entity::from(trigger), *trigger);
        Ok(())
    }
}
