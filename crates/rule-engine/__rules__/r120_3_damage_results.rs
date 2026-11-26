use crate::events;
use bevy::prelude::{MessageReader, MessageWriter};

use crate::Did;
use crate::Resolve;

/// 120.3. Damage may have one or more of the following results, depending on whether the recipient
/// of the damage is a player or permanent, the characteristics of the damage’s source, and the
/// characteristics of the damage’s recipient (if it’s a permanent).
pub fn r120_3_damage_results(
    mut damage_dealt: MessageReader<Resolve<events::DealDamage>>,
    mut did_damage: MessageWriter<Did<events::DealDamage>>,
) {
    did_damage.write_batch(
        damage_dealt.read().filter(|dealt| dealt.amount != 0).map(|dealt| Did(**dealt)),
    );
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn damage_is_announced_so_a_damage_trigger_fires() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.battlefield([SCROLL_THIEF]), P2]);
        let [thief] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(SCROLL_THIEF)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([thief]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
        ])?;

        assert!(game.trigger_sources().contains(&thief.into()));
        Ok(())
    }
}
