use bevy::prelude::EntityCommands;

use crate::cards;
use crate::keywords;

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
    let desired_effective =
        characteristics.power_toughness.filter(|_| characteristics.on_battlefield);
    if desired_effective != current.effective.copied().map(|effective| *effective) {
        if let Some(effective) = desired_effective {
            entity.insert(cards::EffectivePT(effective));
        } else {
            entity.remove::<cards::EffectivePT>();
        }
    }
    let desired = |keyword: keywords::Keyword| {
        characteristics.on_battlefield && characteristics.keywords.contains(&keyword)
    };
    cards::Characteristics::sync_marker(
        entity,
        desired(keywords::deathtouch()),
        current.deathtouch,
        cards::Deathtouch,
    );
    cards::Characteristics::sync_marker(
        entity,
        desired(keywords::infect()),
        current.infect,
        cards::Infect,
    );
    cards::Characteristics::sync_marker(
        entity,
        desired(keywords::first_strike()),
        current.first_strike,
        cards::FirstStrike,
    );
    cards::Characteristics::sync_marker(
        entity,
        desired(keywords::double_strike()),
        current.double_strike,
        cards::DoubleStrike,
    );
    cards::Characteristics::sync_marker(
        entity,
        desired(keywords::lifelink()),
        current.lifelink,
        cards::Lifelink,
    );
    cards::Characteristics::sync_marker(
        entity,
        desired(keywords::trample()),
        current.trample,
        cards::Trample,
    );
    cards::Characteristics::sync_marker(
        entity,
        desired(keywords::flying()),
        current.flying,
        cards::Flying,
    );
    cards::Characteristics::sync_marker(
        entity,
        desired(keywords::reach()),
        current.reach,
        cards::Reach,
    );
    cards::Characteristics::sync_marker(
        entity,
        desired(keywords::vigilance()),
        current.vigilance,
        cards::Vigilance,
    );
    cards::Characteristics::sync_marker(
        entity,
        desired(keywords::defender()),
        current.defender,
        cards::Defender,
    );
    cards::Characteristics::sync_marker(
        entity,
        characteristics.on_battlefield
            && characteristics.types.contains(&cards::CardType::Creature),
        current.creature,
        cards::Creature,
    );
    characteristics.write_triggered_markers(entity, current);
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
        let characteristics = cards::Characteristics::from_printed(&bears, true).with_effects(&[]);
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
    fn defender_keyword_writes_the_defender_marker() {
        let mut app = App::new();
        let wall = (testing::WALL_OF_STONE.definition)();
        let characteristics = cards::Characteristics::from_printed(&wall, true).with_effects(&[]);
        let entity = app.world_mut().spawn(wall).id();
        app.add_systems(
            Update,
            move |mut commands: Commands, markers: Query<cards::CurrentMarkers<'_>>| {
                let current = markers.get(entity).expect("test spawns the card");
                characteristics.write_markers(&mut commands.entity(entity), current);
            },
        );
        app.update();

        let world = app.world();
        assert!(world.get::<cards::Defender>(entity).is_some());
    }

    #[test]
    fn a_creature_on_the_battlefield_gets_the_creature_marker() {
        let mut app = App::new();
        let bears = (testing::GRIZZLY_BEARS.definition)();
        let strength = (testing::HOLY_STRENGTH.definition)();
        let bears_characteristics =
            cards::Characteristics::from_printed(&bears, true).with_effects(&[]);
        let strength_characteristics =
            cards::Characteristics::from_printed(&strength, true).with_effects(&[]);
        let bears_entity = app.world_mut().spawn(bears).id();
        let strength_entity = app.world_mut().spawn(strength).id();
        app.add_systems(
            Update,
            move |mut commands: Commands, markers: Query<cards::CurrentMarkers<'_>>| {
                let bears_current = markers.get(bears_entity).expect("test spawns the card");
                bears_characteristics
                    .write_markers(&mut commands.entity(bears_entity), bears_current);
                let strength_current = markers.get(strength_entity).expect("test spawns the card");
                strength_characteristics
                    .write_markers(&mut commands.entity(strength_entity), strength_current);
            },
        );
        app.update();

        let world = app.world();
        assert!(world.get::<cards::Creature>(bears_entity).is_some());
        assert!(world.get::<cards::Creature>(strength_entity).is_none());
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
                cards::Controller(players[0]),
                damage::Damage(0),
            ))
            .id();
        let mut state = turns::TurnState::from(turns::Turn {
            number: 1,
            active: players[0],
            step: turns::Step::PrecombatMain,
        });
        state.priority = turns::Priority::held_by(players[0]);
        app.insert_resource(state);
        app.insert_resource(turns::TurnOrder::from(players));
        app.update();

        let snapshot = |app: &App| {
            let world = app.world();
            (
                (
                    world.get::<cards::OnBattlefield>(bears).is_some(),
                    world.get::<cards::EffectivePT>(bears).copied().map(|effective| *effective),
                    world.get::<cards::Deathtouch>(bears).is_some(),
                    world.get::<cards::Infect>(bears).is_some(),
                    world.get::<cards::FirstStrike>(bears).is_some(),
                    world.get::<cards::DoubleStrike>(bears).is_some(),
                    world.get::<cards::Lifelink>(bears).is_some(),
                    world.get::<cards::Trample>(bears).is_some(),
                ),
                (
                    world.get::<cards::Flying>(bears).is_some(),
                    world.get::<cards::Reach>(bears).is_some(),
                    world.get::<cards::Vigilance>(bears).is_some(),
                    world.get::<cards::Defender>(bears).is_some(),
                    world.get::<cards::Creature>(bears).is_some(),
                    world.get::<cards::TriggeredEnters>(bears).is_some(),
                    world.get::<cards::TriggeredAtStep>(bears).is_some(),
                ),
            )
        };
        let before = snapshot(&app);
        assert_eq!(
            before.0.1,
            Some(cards::PowerToughness {
                power: 2,
                toughness: 2
            })
        );
        app.world_mut().entity_mut(bears).remove::<(
            cards::OnBattlefield,
            cards::EffectivePT,
            cards::Deathtouch,
            cards::Infect,
            cards::FirstStrike,
            cards::DoubleStrike,
            cards::Lifelink,
            cards::Trample,
            cards::Flying,
            cards::Reach,
            cards::Vigilance,
            cards::Defender,
            cards::Creature,
            cards::TriggeredEnters,
            cards::TriggeredAtStep,
        )>();
        assert_ne!(snapshot(&app), before);
        app.update();
        assert_eq!(snapshot(&app), before);
    }
}
