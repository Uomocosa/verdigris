use crate::testing;
use crate::turns;

pub fn cleanup(game: &mut testing::MockGame) {
    let _ = game
        .app
        .world_mut()
        .write_message(turns::CleanupStepStarted);
    game.app.update();
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::testing;
    use crate::zones;

    use super::cleanup;

    #[test]
    fn test_usage() {
        let mut game = testing::MockGame::builder()
            .battlefield(testing::Seat::P1, [testing::grizzly_bears()])
            .build();
        let bears = game
            .find(game.p1, zones::Zone::Battlefield, "Grizzly Bears")
            .unwrap();
        game.app.world_mut().entity_mut(bears).insert((
            zones::Damage(1),
            effects::PowerToughnessModifier {
                power: 3,
                toughness: 3,
            },
        ));
        cleanup(&mut game);
        assert_eq!(
            game.app.world().get::<zones::Damage>(bears).copied(),
            Some(zones::Damage(0))
        );
    }
}
