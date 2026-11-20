use bevy::prelude::{MessageReader, Query};

use crate::cards;
use crate::damage;
use crate::players;

/// 702.15b Damage dealt by a source with lifelink causes that source’s controller, or its owner if it
/// has no controller, to gain that much life (in addition to any other results that damage causes).
/// See rule 120.3.
pub fn r702_15b_lifelink(
    mut damage_dealt: MessageReader<damage::DamageDealt>,
    mut lives: Query<&mut players::Life>,
    sources: Query<(&cards::CardDefinition, &cards::Owner)>,
) {
    for event in damage_dealt.read() {
        if event.amount == 0 {
            continue;
        }
        let Ok((definition, owner)) = sources.get(event.source) else {
            continue;
        };
        if !definition.keywords.contains(&cards::Keyword::Lifelink) {
            continue;
        }
        if let Ok(mut life) = lives.get_mut(**owner) {
            *life =
                players::Life((**life).saturating_add(damage::Damage::signed_amount(event.amount)));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::GRAVEYARD;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn test_usage() -> TestResult {
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
        assert_eq!(game.zone_of(vampire), GRAVEYARD);
        assert_eq!(game.zone_of(bears), GRAVEYARD);
        Ok(())
    }

    #[test]
    fn r702_15b_lifelink_combat_damage_gains_its_controller_that_much_life() -> TestResult {
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
}
