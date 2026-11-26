use std::collections::BTreeMap;

use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Entity, MessageReader, MessageWriter, Query, ResMut, With, not, on_message};

use crate::Attempt;
use crate::StateBasedActionsPerformed;
use crate::actions;
use crate::cards;
use crate::events;
use crate::players;

/// 704.5j If two or more legendary permanents with the same name are controlled by the same
/// player, that player chooses one of them, and the rest are put into their owners’ graveyards.
/// This is called the “legend rule.”
pub fn r704_5j_legend_rule() -> ScheduleConfigs<ScheduleSystem> {
    (
        put_the_rest_into_graveyards,
        ask_which_to_keep.run_if(not(on_message::<actions::Answered>)),
    )
        .chain()
}

fn ask_which_to_keep(
    mut asks: MessageWriter<actions::Ask>,
    permanents: Query<
        (Entity, &cards::CardDefinition, &cards::Controller),
        With<cards::OnBattlefield>,
    >,
) {
    let mut legendary_by_controller_and_name: BTreeMap<(players::PlayerId, &str), Vec<Entity>> =
        BTreeMap::new();
    for (entity, definition, controller) in permanents.iter() {
        if definition.type_line.supertypes.contains(&cards::Supertype::Legendary) {
            legendary_by_controller_and_name
                .entry((**controller, definition.name.as_str()))
                .or_default()
                .push(entity);
        }
    }
    for ((controller, _), entities) in legendary_by_controller_and_name {
        if entities.len() < 2 {
            continue;
        }
        asks.write(actions::Ask(actions::Choice::Legendary(
            actions::LegendaryChoice {
                player: controller,
                legal: entities.into_iter().collect(),
            },
        )));
    }
}

fn put_the_rest_into_graveyards(
    mut answers: MessageReader<actions::Answered>,
    mut burials: MessageWriter<Attempt<events::PutIntoGraveyard>>,
    mut performed: ResMut<StateBasedActionsPerformed>,
    permanents: Query<Entity, With<cards::OnBattlefield>>,
) {
    for answered in answers.read() {
        let actions::Choice::Legendary(choice) = &answered.choice else {
            continue;
        };
        let actions::Action::ChooseLegendary(chosen) = &answered.answer else {
            continue;
        };
        for other in choice.legal.iter().copied().filter(|other| other != chosen) {
            if permanents.get(other).is_err() {
                continue;
            }
            burials.write(Attempt(events::PutIntoGraveyard { object: other }));
        }
        **performed = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::GRAVEYARD;
    use crate::actions;
    use crate::testing::*;

    #[test]
    fn legend_rule_keeps_the_chosen_permanent_and_buries_the_other() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([
            P1.battlefield([ISAMARU_HOUND_OF_KONDA, ISAMARU_HOUND_OF_KONDA]),
            P2,
        ]);
        let [isamaru_1, isamaru_2] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(ISAMARU_HOUND_OF_KONDA)?;

        game.play([p1.pass_priority()])?;
        game.play([p1.keep_legendary(isamaru_1)])?;

        assert!(game.player(p1).cards_in(BATTLEFIELD).has(isamaru_1));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(isamaru_2));
        assert!(game.app.world().resource::<actions::PendingChoice>().is_none());
        Ok(())
    }

    #[test]
    fn a_cant_be_destroyed_effect_does_not_save_the_other_legend() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([
            P1.battlefield([
                ISAMARU_HOUND_OF_KONDA,
                ISAMARU_HOUND_OF_KONDA,
                TEST_SAFE_HAVEN,
            ]),
            P2,
        ]);
        let [isamaru_1, isamaru_2] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(ISAMARU_HOUND_OF_KONDA)?;

        game.play([p1.pass_priority()])?;
        game.play([p1.keep_legendary(isamaru_1)])?;

        assert!(game.player(p1).cards_in(BATTLEFIELD).has(isamaru_1));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(isamaru_2));
        Ok(())
    }
}
