use crate::{HAND, STACK};
use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, Res};

use crate::actions;
use crate::cards;
use crate::rules::helpers;
use crate::turns;
use crate::zones;

/// 601.2a To propose the casting of a spell, a player first moves that card (or that copy of a
/// card) from where it is to the stack. It becomes the topmost object on the stack. It has all
/// the characteristics of the card (or the copy of a card) associated with it, and that player
/// becomes its controller. The spell remains on the stack until it resolves, it's countered, or
/// a rule or effect moves it elsewhere.
pub fn r601_2a_cast(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut commands: Commands,
    priority: Res<turns::Priority>,
    turn: Res<turns::Turn>,
    cards: Query<(&cards::CardDefinition, &cards::Owner, &zones::InZone)>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
) {
    let priority = priority.into_inner();
    let turn = turn.into_inner();
    for action in actions.read() {
        let actions::Action::Cast(spell) = &action.action else {
            continue;
        };
        let spell = *spell;
        if action.player != priority.player {
            rejected.write(actions::Rejected::CannotCast { spell });
            continue;
        }
        let Ok((definition, owner, in_zone)) = cards.get(spell) else {
            rejected.write(actions::Rejected::CannotCast { spell });
            continue;
        };
        let is_in_casters_hand = zone_entities
            .get(**in_zone)
            .is_ok_and(|(_, zone, zone_owner)| {
                *zone == HAND && zone_owner.is_some_and(|zone_owner| **zone_owner == action.player)
            });
        if **owner != action.player || !is_in_casters_hand {
            rejected.write(actions::Rejected::CannotCast { spell });
            continue;
        }
        let Some(stack) = helpers::find_zone(STACK, None, &zone_entities) else {
            rejected.write(actions::Rejected::CannotCast { spell });
            continue;
        };
        let is_instant = definition.types.contains(&cards::CardType::Instant);
        if !is_instant {
            let do_stack_is_empty = !cards.iter().any(|(_, _, zone)| **zone == stack);
            let is_main_phase = matches!(
                turn.step,
                turns::Step::PrecombatMain | turns::Step::PostcombatMain
            );
            let is_active_player = turn.active == action.player;
            let do_sorcery_speed_conditions_hold =
                do_stack_is_empty && is_main_phase && is_active_player;
            if !do_sorcery_speed_conditions_hold {
                rejected.write(actions::Rejected::CannotCast { spell });
                continue;
            }
        }
        commands.entity(spell).insert(zones::InZone(stack));
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{HAND, STACK};

    #[test]
    fn test_usage() {
        let (mut game, [p1, ..]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let bolt = game.player(p1).cards_in(HAND).only(LIGHTNING_BOLT).unwrap();
        game.play([p1.cast(bolt)]).unwrap();
        assert_eq!(game.zone_of(bolt), STACK);
    }

    #[test]
    fn sorcery_cannot_be_cast_on_an_opponents_turn() {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(2, turns::Step::PrecombatMain)
            .players([P1.hand([DIVINATION]).mana_pool(mana![2, U]), P2.active()]);
        game.play([p2.pass_priority()]).unwrap();
        let divination = game.player(p1).cards_in(HAND).only(DIVINATION).unwrap();
        let err = game.play([p1.cast(divination)]).unwrap_err();
        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::CannotCast {
                spell: divination.into()
            })
        );
        assert_eq!(game.zone_of(divination), HAND);
    }

    #[test]
    fn sorcery_cannot_be_cast_while_the_stack_is_not_empty() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, DIVINATION])
                .mana_pool(mana![R, 2, U]),
            P2,
        ]);
        let bolt = game.player(p1).cards_in(HAND).only(LIGHTNING_BOLT).unwrap();
        game.play([p1.cast(bolt).targeting(p2)]).unwrap();
        let divination = game.player(p1).cards_in(HAND).only(DIVINATION).unwrap();
        let err = game.play([p1.cast(divination)]).unwrap_err();
        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::CannotCast {
                spell: divination.into()
            })
        );
        assert_eq!(game.zone_of(divination), HAND);
    }
}
