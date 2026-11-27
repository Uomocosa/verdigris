use bevy::prelude::{Commands, Entity, Query, Without};

use crate::cards;

/// 506.4. A permanent is removed from combat if it leaves the battlefield, if its controller
/// changes, if it phases out, if an effect specifically removes it from combat, if it’s a
/// planeswalker that’s being attacked and stops being a planeswalker, if it’s a battle that’s being
/// attacked and stops being a battle, or if it’s an attacking or blocking creature that regenerates
/// (see rule 701.19), stops being a creature, or becomes a battle. A creature that’s removed from
/// combat stops being an attacking, blocking, blocked, and/or unblocked creature. A planeswalker or
/// battle that’s removed from combat stops being attacked.
pub fn r506_4_leaves_the_battlefield(
    mut commands: Commands,
    left_the_battlefield: Query<Entity, (cards::InCombat, Without<cards::OnBattlefield>)>,
) {
    for permanent in &left_the_battlefield {
        commands.entity(permanent).remove::<(
            cards::Attacking,
            cards::Blocking,
            cards::Blocked,
            cards::HadFirstStrikeAsDamageBegan,
        )>();
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn a_creature_that_leaves_the_battlefield_is_removed_from_combat() -> TestResult {
        use crate::{GRAVEYARD, HAND};

        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([GRIZZLY_BEARS, MOUNTAIN]).hand([LIGHTNING_BOLT]),
            P2,
        ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [mountain] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(MOUNTAIN)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([bears]),
        ])?;
        game.play([
            p1.activate_ability(mountain.ability()).cast(bolt).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;

        assert!(game.card(bears).is_in(GRAVEYARD).of(p1));
        assert!(!game.is_attacking(bears));
        Ok(())
    }
}
