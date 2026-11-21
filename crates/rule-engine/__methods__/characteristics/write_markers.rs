use bevy::prelude::EntityCommands;

use crate::cards;

pub fn write_markers(
    characteristics: &cards::Characteristics,
    entity: &mut EntityCommands,
    current: cards::CurrentMarkersItem<'_, '_, '_>,
) {
    cards::Characteristics::sync_marker(
        entity,
        characteristics.on_battlefield,
        current.on_battlefield,
        cards::OnBattlefield,
    );
    let desired_effective = if characteristics.on_battlefield {
        characteristics.power_toughness
    } else {
        None
    };
    if desired_effective != current.effective.copied().map(|effective| *effective) {
        if let Some(effective) = desired_effective {
            entity.insert(cards::EffectivePT(effective));
        } else {
            entity.remove::<cards::EffectivePT>();
        }
    }
    let desired = |keyword: cards::Keyword| {
        characteristics.on_battlefield && characteristics.keywords.contains(&keyword)
    };
    cards::Characteristics::sync_marker(
        entity,
        desired(cards::Keyword::Indestructible),
        current.indestructible,
        cards::Indestructible,
    );
    cards::Characteristics::sync_marker(
        entity,
        desired(cards::Keyword::Deathtouch),
        current.deathtouch,
        cards::Deathtouch,
    );
    cards::Characteristics::sync_marker(
        entity,
        desired(cards::Keyword::Lifelink),
        current.lifelink,
        cards::Lifelink,
    );
    cards::Characteristics::sync_marker(
        entity,
        desired(cards::Keyword::Trample),
        current.trample,
        cards::Trample,
    );
    cards::Characteristics::sync_marker(
        entity,
        desired(cards::Keyword::Flying),
        current.flying,
        cards::Flying,
    );
    cards::Characteristics::sync_marker(
        entity,
        desired(cards::Keyword::Reach),
        current.reach,
        cards::Reach,
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Commands, Query, Update};

    use crate::BATTLEFIELD;
    use crate::RulesPlugin;
    use crate::cards;
    use crate::damage;
    use crate::pt;
    use crate::testing;
    use crate::turns;
    use crate::zones;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        let bears = (testing::GRIZZLY_BEARS.definition)();
        let characteristics =
            cards::Characteristics::from_printed(&bears, true).with_modifier(None);
        let entity = app.world_mut().spawn(bears).id();
        app.add_systems(
            Update,
            move |mut commands: Commands, markers: Query<cards::CurrentMarkers<'_>>| {
                let current = markers.get(entity).expect("test spawns the card");
                characteristics.write_markers(&mut commands.entity(entity), current);
            },
        );
        app.update();

        let world = app.world();
        assert!(world.get::<cards::OnBattlefield>(entity).is_some());
        assert_eq!(
            world.get::<cards::EffectivePT>(entity).copied(),
            Some(cards::EffectivePT(pt!(2, 2)))
        );
        assert!(world.get::<cards::Flying>(entity).is_none());
    }

    #[test]
    fn removing_derived_components_and_rerunning_layers_is_identity() {
        let mut app = App::new();
        app.add_plugins(RulesPlugin);
        let players =
            testing::MockGameBuilder::spawn_players(&mut app, &[testing::P1, testing::P2]);
        let battlefield = app.world_mut().spawn((BATTLEFIELD, cards::Owner(players[0]))).id();
        let bears = app
            .world_mut()
            .spawn((
                (testing::GRIZZLY_BEARS.definition)(),
                zones::InZone(battlefield),
                cards::Owner(players[0]),
                damage::Damage(0),
            ))
            .id();
        app.insert_resource(turns::Turn {
            number: 1,
            active: players[0],
            step: turns::Step::PrecombatMain,
        });
        app.insert_resource(turns::Priority {
            player: players[0],
            consecutive_passes: 0,
        });
        app.insert_resource(turns::TurnOrder::from(players));
        app.update();

        let snapshot = |app: &App| {
            let world = app.world();
            (
                world.get::<cards::OnBattlefield>(bears).is_some(),
                world.get::<cards::EffectivePT>(bears).copied().map(|effective| *effective),
                world.get::<cards::Indestructible>(bears).is_some(),
                world.get::<cards::Deathtouch>(bears).is_some(),
                world.get::<cards::Lifelink>(bears).is_some(),
                world.get::<cards::Trample>(bears).is_some(),
                world.get::<cards::Flying>(bears).is_some(),
                world.get::<cards::Reach>(bears).is_some(),
            )
        };
        let before = snapshot(&app);
        assert_eq!(
            before.1,
            Some(cards::PowerToughness {
                power: 2,
                toughness: 2
            })
        );
        app.world_mut().entity_mut(bears).remove::<(
            cards::OnBattlefield,
            cards::EffectivePT,
            cards::Indestructible,
            cards::Deathtouch,
            cards::Lifelink,
            cards::Trample,
            cards::Flying,
            cards::Reach,
        )>();
        assert_ne!(snapshot(&app), before);
        app.update();
        assert_eq!(snapshot(&app), before);
    }
}
