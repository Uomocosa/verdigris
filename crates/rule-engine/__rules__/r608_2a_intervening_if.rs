use crate::STACK;
use bevy::prelude::{Commands, Entity, MessageReader, Query};

use crate::players;
use crate::stack;
use crate::zones;

/// 608.2a If a triggered ability has an intervening “if” clause, it checks whether the clause’s
/// condition is true. If it isn’t, the ability is removed from the stack and does nothing.
/// Otherwise, it continues to resolve. See rule 603.4.
pub fn r608_2a_intervening_if(
    mut resolutions: MessageReader<stack::Resolution>,
    mut commands: Commands,
    abilities: Query<(&stack::TriggeredAbilityOnStack, &zones::InZone)>,
    lives: Query<(Entity, &players::Life)>,
    zones: zones::ZoneLookup,
) {
    for resolution in resolutions.read() {
        let spell = **resolution;
        let Ok((ability, in_zone)) = abilities.get(spell) else {
            continue;
        };
        if !zones.contains(STACK, **in_zone) {
            continue;
        }
        let Some(condition) = ability.intervening_if else {
            continue;
        };
        if !condition.holds(ability.controller, &lives) {
            commands.entity(spell).despawn();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, HAND};

    #[test]
    fn felidar_condition_is_rechecked_on_resolution() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::builder().turn(1, turns::Step::Untap).players([
            P1.battlefield([FELIDAR_SOVEREIGN]).life(40),
            P2.battlefield([MOUNTAIN]).hand([LIGHTNING_BOLT]),
        ]);
        let [sovereign] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FELIDAR_SOVEREIGN)?;
        let [mountain] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(MOUNTAIN)?;
        let [bolt] = game.player(p2).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.pass_priority(), p1.pass_priority()])?;
        assert_eq!(game.trigger_sources().len(), 1);
        game.play([
            p2.activate_ability(mountain.ability()).cast(bolt).targeting(p1).pass_priority(),
            p1.pass_priority(),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;
        assert_eq!(game.life(p1), 37);
        assert!(!game.has_won(p1));
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(sovereign));
        Ok(())
    }
}
