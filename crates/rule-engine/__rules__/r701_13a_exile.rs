use crate::events;
use bevy::prelude::{Commands, MessageReader, ResMut};

use crate::EXILE;
use crate::Resolve;
use crate::StateBasedActionsPerformed;
use crate::zones;

/// 701.13a To exile an object, move it to the exile zone from wherever it is. See rule 406, “Exile.”
pub fn r701_13a_exile(
    mut exiles: MessageReader<Resolve<events::Exile>>,
    mut commands: Commands,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
    zones: zones::ZoneLookup,
) {
    for event in exiles.read() {
        let Some(exile) = zones.find(EXILE, Some(event.owner)) else {
            continue;
        };
        commands.entity(event.object).insert(zones::InZone(exile));
        **state_based_actions_performed = true;
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, CLEANUP, GRAVEYARD, HAND};

    #[test]
    fn exiled_knight_tokens_move_to_their_owners_exile_zone() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([WAYLAY]).mana_pool(mana![2, W]), P2]);
        let [waylay] = game.player(p1).cards_in(HAND).exactly::<1>(WAYLAY)?;
        game.play([p1.cast(waylay).pass_priority(), p2.pass_priority()])?;
        let [knight_1, knight_2, knight_3] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<3>(KNIGHT_TOKEN)?;

        game.play([p1.pass_to(CLEANUP), p2.pass_to(CLEANUP)])?;
        assert_eq!(game.turn().step, turns::Step::Cleanup);
        game.play([p1.pass_priority()])?;
        game.play([p1.order_triggers([knight_1, knight_2, knight_3])])?;
        for _ in 0..3 {
            game.play([p1.pass_priority(), p2.pass_priority()])?;
        }

        assert!(game.player(p1).cards_in(BATTLEFIELD).has_no(knight_1));
        assert!(game.player(p1).cards_in(BATTLEFIELD).has_no(knight_2));
        assert!(game.player(p1).cards_in(BATTLEFIELD).has_no(knight_3));
        assert!(game.player(p1).cards_in(GRAVEYARD).has_no(knight_1));
        assert!(game.app.world().get_entity(Entity::from(knight_1)).is_err());
        Ok(())
    }
}
