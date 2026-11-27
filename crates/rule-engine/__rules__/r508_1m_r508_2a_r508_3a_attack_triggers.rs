use bevy::prelude::{MessageReader, Query, ResMut};

use crate::Did;
use crate::abilities;
use crate::cards;
use crate::events;
use crate::rules;
use crate::stack;

/// 508.1m Any abilities that trigger on attackers being declared trigger.
///
/// 508.2a Abilities that trigger on a creature attacking trigger only at the point the creature
/// is declared as an attacker. They will not trigger if a creature attacks and then that
/// creature’s characteristics change to match the ability’s trigger condition.
///
/// 508.3a An ability that reads “Whenever [a creature] attacks, . . .” triggers if that creature
/// is declared as an attacker. Similarly, “Whenever [a creature] attacks [a player,
/// planeswalker, or battle], . . .” triggers if that creature is declared as an attacker
/// attacking that player or permanent. Such abilities won’t trigger if a creature is put onto
/// the battlefield attacking.
pub fn r508_1m_r508_2a_r508_3a_attack_triggers(
    mut declared: MessageReader<Did<events::DeclareAttackers>>,
    mut queue: ResMut<stack::TriggerQueue>,
    attackers: Query<(&cards::CardDefinition, &cards::Controller)>,
    answers: rules::Rules,
) {
    for declared in declared.read() {
        for attacker in &declared.attackers {
            let Ok((definition, _)) = attackers.get(*attacker) else {
                continue;
            };
            for ability in definition.triggered_abilities() {
                if !matches!(ability.event, abilities::TriggerEvent::Attacks) {
                    continue;
                }
                queue.push_checked(*attacker, ability.clone(), &answers);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::pt;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn an_attack_trigger_waits_on_the_stack_then_pumps_the_attacker() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([BORDERLAND_MARAUDER]), P2]);
        let [marauder] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(BORDERLAND_MARAUDER)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([marauder]),
        ])?;

        assert!(game.trigger_sources().contains(&marauder.into()));
        assert_eq!(game.pt(marauder), Some(pt!(1, 2)));

        game.play([p1.pass_priority(), p2.pass_priority()])?;

        assert_eq!(game.pt(marauder), Some(pt!(3, 2)));
        assert!(game.card(marauder).is_in(BATTLEFIELD).of(p1));
        Ok(())
    }

    #[test]
    fn a_pumped_attacker_deals_three_damage_to_the_defending_player() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([BORDERLAND_MARAUDER]), P2]);
        let [marauder] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(BORDERLAND_MARAUDER)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([marauder]),
        ])?;
        game.play([p1.pass_priority(), p2.pass_priority()])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.player(p2).life(), 17);
        Ok(())
    }

    #[test]
    fn an_illegal_declaration_triggers_nothing() -> TestResult {
        use crate::actions;

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([BORDERLAND_MARAUDER]),
            P2.battlefield([BORDERLAND_MARAUDER]),
        ]);
        let [marauder] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(BORDERLAND_MARAUDER)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
        ])?;

        game.act(p1, actions::Action::DeclareAttackers(vec![marauder.into()]));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![marauder.into()]
            })
        );
        assert_eq!(game.trigger_sources().len(), 0);
        Ok(())
    }
}
