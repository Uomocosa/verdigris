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
    cards: Query<(&cards::CardDefinition, &zones::Owner, &zones::InZone)>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&zones::Owner>)>,
) {
    let priority = priority.into_inner();
    let turn = turn.into_inner();
    for action in actions.read() {
        let actions::Action::Cast(spell) = &action.action else {
            continue;
        };
        let spell = *spell;
        if action.player != **priority {
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
                *zone == zones::Zone::Hand
                    && zone_owner.is_some_and(|zone_owner| **zone_owner == action.player)
            });
        if **owner != action.player || !is_in_casters_hand {
            rejected.write(actions::Rejected::CannotCast { spell });
            continue;
        }
        let Some(stack) = helpers::find_zone(zones::Zone::Stack, None, &zone_entities) else {
            rejected.write(actions::Rejected::CannotCast { spell });
            continue;
        };
        let is_instant = definition.types.contains(&cards::CardType::Instant);
        if !is_instant {
            let do_stack_is_empty = !cards.iter().any(|(_, _, zone)| **zone == stack);
            let is_main_phase = matches!(
                turn.phase,
                turns::Phase::PrecombatMain | turns::Phase::PostcombatMain
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
    use crate::testing;
    use crate::turns;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder()
            .hand(testing::Seat::P1, [testing::lightning_bolt()])
            .mana_pool(testing::Seat::P1, mana![R])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(bolt));
        assert_eq!(game.zone_of(bolt), zones::Zone::Stack);
    }

    #[test]
    fn sorcery_cannot_be_cast_on_an_opponents_turn() {
        let mut game = testing::MockGame::builder()
            .turn(2, testing::Seat::P2, turns::Phase::PrecombatMain)
            .hand(testing::Seat::P1, [testing::divination()])
            .mana_pool(testing::Seat::P1, mana![2, U])
            .build();
        let p1 = game.p1;
        game.give_priority(p1);
        let divination = game.find(p1, zones::Zone::Hand, "Divination").unwrap();
        game.act(p1, actions::Action::Cast(divination));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotCast { spell: divination })
        );
        assert_eq!(game.zone_of(divination), zones::Zone::Hand);
    }

    #[test]
    fn sorcery_cannot_be_cast_while_the_stack_is_not_empty() {
        let mut game = testing::MockGame::builder()
            .hand(
                testing::Seat::P1,
                [testing::lightning_bolt(), testing::divination()],
            )
            .mana_pool(testing::Seat::P1, mana![R, 2, U])
            .build();
        let bolt = game
            .find(game.p1, zones::Zone::Hand, "Lightning Bolt")
            .unwrap();
        game.act(game.p1, actions::Action::Cast(bolt));
        game.act(game.p1, actions::Action::ChooseTargets(vec![game.p2]));
        let divination = game.find(game.p1, zones::Zone::Hand, "Divination").unwrap();
        game.act(game.p1, actions::Action::Cast(divination));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotCast { spell: divination })
        );
        assert_eq!(game.zone_of(divination), zones::Zone::Hand);
    }
}
