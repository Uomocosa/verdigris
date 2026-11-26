use crate::events;
use bevy::prelude::{MessageReader, MessageWriter, Query, With};

use crate::Attempt;
use crate::Resolve;
use crate::cards;

/// 702.15b Damage dealt by a source with lifelink causes that source’s controller, or its owner if
/// it has no controller, to gain that much life (in addition to any other results that damage
/// causes). See rule 120.3.
pub fn r702_15b_lifelink(
    mut damage_dealt: MessageReader<Resolve<events::DealDamage>>,
    mut life_gained: MessageWriter<Attempt<events::GainLife>>,
    sources: Query<&cards::Controller, With<cards::Lifelink>>,
) {
    for event in damage_dealt.read() {
        let Ok(controller) = sources.get(event.source) else {
            continue;
        };
        life_gained.write(Attempt(events::GainLife {
            player: **controller,
            amount: event.amount,
        }));
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::GRAVEYARD;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn lifelink_gains_life_even_when_source_dies() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([CHILD_OF_NIGHT]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [vampire] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(CHILD_OF_NIGHT)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([vampire]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([p1.pass_priority(), p2.declare_blockers([(bears, vampire)])])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.life(p1), 22);
        assert!(game.player(p1).cards_in(GRAVEYARD).has(vampire));
        assert!(game.player(p2).cards_in(GRAVEYARD).has(bears));
        Ok(())
    }

    #[test]
    fn lifelink_combat_damage_gains_its_controller_that_much_life() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([CHILD_OF_NIGHT]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [vampire] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(CHILD_OF_NIGHT)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([vampire]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.life(p2), 18);
        assert_eq!(game.life(p1), 22);
        Ok(())
    }

    #[test]
    fn a_player_who_cant_gain_life_gains_none_from_lifelink() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([CHILD_OF_NIGHT, TEST_CANT_DISCARD]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [vampire] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(CHILD_OF_NIGHT)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([vampire]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.life(p2), 18);
        assert_eq!(game.life(p1), 20);
        Ok(())
    }
}
