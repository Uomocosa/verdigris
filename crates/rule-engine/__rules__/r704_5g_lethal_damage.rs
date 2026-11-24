use bevy::prelude::{Entity, MessageWriter, Query, With};

use crate::cards;
use crate::damage;
use crate::rules;

/// 704.5g If a creature has toughness greater than 0, it has damage marked on it, and the total
/// damage marked on it is greater than or equal to its toughness, that creature has been dealt
/// lethal damage and is destroyed. Regeneration can replace this event.
pub fn r704_5g_lethal_damage(
    mut destructions: MessageWriter<rules::Would<rules::Destroy>>,
    cards: Query<
        (Entity, &cards::Owner, &cards::EffectivePT, &damage::Damage),
        With<cards::OnBattlefield>,
    >,
) {
    for (creature, owner, effective, damage) in cards.iter() {
        let marked = damage::Damage::signed_amount(**damage);
        let is_lethal = effective.toughness > 0 && marked > 0 && marked >= effective.toughness;
        if !is_lethal {
            continue;
        }
        destructions.write(rules::Would(rules::Destroy {
            permanent: creature,
            controller: **owner,
        }));
    }
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn bolt_with_lethal_damage_destroys_bears() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;
        game.play([
            p1.cast(bolt).targeting(bears).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.player(p2).cards_in(GRAVEYARD).has(bears));
        Ok(())
    }
}
