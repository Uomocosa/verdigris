use crate::events;
use bevy::prelude::{Commands, MessageReader, Query};

use crate::Resolve;
use crate::damage;
use crate::keywords;
use crate::players;
use crate::rules;

/// 120.3e Damage dealt to a creature by a source with neither wither nor infect causes that much
/// damage to be marked on that creature.
pub fn r120_3e_damage_to_a_creature(
    mut damage_dealt: MessageReader<Resolve<events::DealDamage>>,
    mut commands: Commands,
    players: players::Players,
    answers: rules::Rules,
    targets: Query<Option<&damage::Damage>>,
) {
    for event in damage_dealt.read() {
        if players.as_player(event.recipient).is_some() {
            continue;
        }
        let marked = targets.get(event.recipient).ok().flatten().map_or(0, |damage| **damage);
        commands
            .entity(event.recipient)
            .insert(damage::Damage(marked.saturating_add(event.amount)));
        if answers.eval(
            &rules::has_keyword(keywords::deathtouch::ID).of(events::DealDamage::SOURCE),
            &**event,
            None,
        ) {
            commands.entity(event.recipient).insert(damage::DealtByDeathtouch);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::GRAVEYARD;
    use crate::HAND;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn lightning_bolt_marks_damage_and_destroys_the_creature() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::builder().turn(1, turns::Step::PrecombatMain).players([
                P1.battlefield([WALKING_CORPSE]),
                P2.active().hand([LIGHTNING_BOLT]).mana_pool(mana![R]),
            ]);
        let [corpse] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WALKING_CORPSE)?;
        let [bolt] = game.player(p2).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([
            p2.cast(bolt).targeting(corpse).pass_priority(),
            p1.pass_priority(),
        ])?;
        assert!(game.moved(corpse).to(GRAVEYARD).of(p1));
        assert_eq!(game.life(p1), 20);
        Ok(())
    }
}
