use bevy::prelude::EntityCommands;

use crate::cards;

pub fn write_markers(
    characteristics: &cards::Characteristics,
    entity: &mut EntityCommands,
    current: (
        bool,
        Option<cards::PowerToughness>,
        bool,
        bool,
        bool,
        bool,
        bool,
        bool,
    ),
) {
    let (
        current_on_battlefield,
        current_effective,
        current_indestructible,
        current_deathtouch,
        current_lifelink,
        current_trample,
        current_flying,
        current_reach,
    ) = current;
    if characteristics.on_battlefield != current_on_battlefield {
        if characteristics.on_battlefield {
            entity.insert(cards::OnBattlefield);
        } else {
            entity.remove::<cards::OnBattlefield>();
        }
    }
    let desired_effective = if characteristics.on_battlefield {
        characteristics.power_toughness
    } else {
        None
    };
    if desired_effective != current_effective {
        if let Some(effective) = desired_effective {
            entity.insert(cards::EffectivePT(effective));
        } else {
            entity.remove::<cards::EffectivePT>();
        }
    }
    let desired = |keyword: cards::Keyword| {
        characteristics.on_battlefield && characteristics.keywords.contains(&keyword)
    };
    if desired(cards::Keyword::Indestructible) != current_indestructible {
        if desired(cards::Keyword::Indestructible) {
            entity.insert(cards::Indestructible);
        } else {
            entity.remove::<cards::Indestructible>();
        }
    }
    if desired(cards::Keyword::Deathtouch) != current_deathtouch {
        if desired(cards::Keyword::Deathtouch) {
            entity.insert(cards::Deathtouch);
        } else {
            entity.remove::<cards::Deathtouch>();
        }
    }
    if desired(cards::Keyword::Lifelink) != current_lifelink {
        if desired(cards::Keyword::Lifelink) {
            entity.insert(cards::Lifelink);
        } else {
            entity.remove::<cards::Lifelink>();
        }
    }
    if desired(cards::Keyword::Trample) != current_trample {
        if desired(cards::Keyword::Trample) {
            entity.insert(cards::Trample);
        } else {
            entity.remove::<cards::Trample>();
        }
    }
    if desired(cards::Keyword::Flying) != current_flying {
        if desired(cards::Keyword::Flying) {
            entity.insert(cards::Flying);
        } else {
            entity.remove::<cards::Flying>();
        }
    }
    if desired(cards::Keyword::Reach) != current_reach {
        if desired(cards::Keyword::Reach) {
            entity.insert(cards::Reach);
        } else {
            entity.remove::<cards::Reach>();
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Commands, Update};

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
        app.add_systems(Update, move |mut commands: Commands| {
            characteristics.write_markers(
                &mut commands.entity(entity),
                (false, None, false, false, false, false, false, false),
            );
        });
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
