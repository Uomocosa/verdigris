use bevy::prelude::{Commands, Entity, Query, ResMut, With, Without};

use crate::BATTLEFIELD;
use crate::StateBasedActionsPerformed;
use crate::cards;
use crate::zones;

type ZonelessToken<'w, 's> = Query<
    'w,
    's,
    Entity,
    (
        With<cards::Token>,
        Without<zones::InZone>,
        Without<zones::LeftZone>,
    ),
>;

/// 704.5d If a token is in a zone other than the battlefield, it ceases to exist.
pub fn r704_5d_token_outside_battlefield(
    mut commands: Commands,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
    tokens: Query<(Entity, &zones::InZone), With<cards::Token>>,
    zoneless_tokens: ZonelessToken<'_, '_>,
    zones: zones::ZoneLookup,
) {
    for (token, in_zone) in tokens.iter() {
        if zones.contains(BATTLEFIELD, **in_zone) {
            continue;
        }
        commands.entity(token).despawn();
        **state_based_actions_performed = true;
    }
    for token in &zoneless_tokens {
        commands.entity(token).despawn();
        **state_based_actions_performed = true;
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::mana;
    use crate::testing::*;
    use crate::zones;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn a_token_killed_by_lightning_bolt_ceases_to_exist() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([RAISE_THE_ALARM, LIGHTNING_BOLT]).mana_pool(mana![1, W, R]),
            P2,
        ]);
        let [raise_the_alarm] = game.player(p1).cards_in(HAND).exactly::<1>(RAISE_THE_ALARM)?;
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.cast(raise_the_alarm).pass_priority(), p2.pass_priority()])?;

        let [soldier, survivor] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(SOLDIER_TOKEN)?;
        game.play([
            p1.cast(bolt).targeting(soldier).pass_priority(),
            p2.pass_priority(),
        ])?;

        assert!(game.card(survivor).is_in(BATTLEFIELD).of(p1));
        assert!(!game.card(soldier).is_in(BATTLEFIELD).of(p1));
        assert!(!game.card(soldier).is_in(GRAVEYARD).of(p1));
        let burial_record = game
            .get::<zones::LeftZone>(Entity::from(soldier))
            .expect("buried token leaves a record");
        assert!(game.app.world().get_entity(*burial_record).is_err());
        assert!(game.moved(bolt).to(GRAVEYARD).of(p1));
        Ok(())
    }
}
