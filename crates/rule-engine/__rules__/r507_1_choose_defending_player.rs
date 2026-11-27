use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{Commands, Entity, MessageReader, MessageWriter, Query, With};

use crate::DEFENDING_PLAYER;
use crate::Game;
use crate::actions;
use crate::effects;
use crate::events;
use crate::players;
use crate::rules;
use crate::turns;

/// 507.1. First, if the game being played is a multiplayer game in which the active player’s
/// opponents don’t all automatically become defending players, the active player chooses one of
/// their opponents. That player becomes the defending player. This turn-based action doesn’t use
/// the stack. (See rule 506.2.)
///
/// # See also
///
/// 506.2a During the combat phase of a multiplayer game, there may be one or more defending
/// players, depending on the variant being played and the options chosen for it. Unless all the
/// attacking player’s opponents automatically become defending players during the combat phase,
/// the attacking player chooses one of their opponents as a turn-based action during the
/// beginning of combat step. (Note that the choice may be dictated by the variant being played or
/// the options chosen for it.) That player becomes the defending player. See rule 802, “Attack
/// Multiple Players Option,” rule 803, “Attack Left and Attack Right Options,” and rule 809,
/// “Emperor Variant.”
pub fn r507_1_choose_defending_player() -> ScheduleConfigs<ScheduleSystem> {
    (
        ask_which_opponent.run_if(turns::Step::BeginningOfCombat.begins()),
        mark_the_chosen_opponent,
    )
        .chain()
}

fn ask_which_opponent(mut asks: MessageWriter<actions::Ask>, game: Game, answers: rules::Rules) {
    let Some(attacking_player) = game.active_player() else {
        return;
    };
    let choice = actions::PickChoice {
        id: DEFENDING_PLAYER,
        player: attacking_player.id,
        source: None,
        among: actions::Candidates::Matching {
            filter: effects::Filter::player()
                & effects::Filter::controlled_by(rules::Players::Opponents),
            context: events::AbilityContext {
                spell_or_ability: *attacking_player.id,
            },
            check: actions::CandidateCheck::None,
        },
        count: actions::Count::exactly(1),
        ordered: false,
    };
    let opponents = choice
        .candidates(&answers)
        .into_iter()
        .filter(|(_, legality)| *legality == rules::Legality::Legal)
        .count();
    if opponents < 2 {
        return;
    }
    asks.write(actions::Ask(actions::Choice::Pick(choice)));
}

fn mark_the_chosen_opponent(
    mut answers: MessageReader<actions::Answered>,
    mut commands: Commands,
    defending: Query<Entity, With<players::DefendingPlayer>>,
    players: players::Players,
) {
    for answered in answers.read() {
        let actions::Choice::Pick(choice) = &answered.choice else {
            continue;
        };
        if choice.id != DEFENDING_PLAYER {
            continue;
        }
        let actions::Action::Choose(chosen) = &answered.answer else {
            continue;
        };
        let [chosen] = chosen.as_slice() else {
            continue;
        };
        let chosen = players::PlayerId(*chosen);
        if !players.is_still_in_the_game(chosen) {
            continue;
        }
        for player in defending.iter() {
            commands.entity(player).remove::<players::DefendingPlayer>();
        }
        commands.entity(*chosen).insert(players::DefendingPlayer);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use bevy::ecs::system::RunSystemOnce;
    use bevy::prelude::Entity;

    use crate::BATTLEFIELD;
    use crate::DEFENDING_PLAYER;
    use crate::actions;
    use crate::players;
    use crate::rules;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn the_active_player_chooses_one_opponent_as_the_defending_player() -> TestResult {
        let (mut game, [p1, p2, p3]) =
            MockGame::builder().turn(2, turns::Step::PrecombatMain).players([
                P1,
                P2.active().battlefield([GRIZZLY_BEARS]),
                P3.battlefield([GRIZZLY_BEARS]),
            ]);
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.play([
            p2.pass_to(turns::Step::BeginningOfCombat),
            p1.pass_to(turns::Step::BeginningOfCombat),
            p3.pass_to(turns::Step::BeginningOfCombat),
        ])?;
        let actions::Choice::Pick(choice) = game
            .app
            .world()
            .get_resource::<actions::PendingChoice>()
            .and_then(|pending| (**pending).clone())
            .expect("a pending defending-player choice")
        else {
            panic!("expected a pick choice");
        };
        assert_eq!(choice.id, DEFENDING_PLAYER);
        assert_eq!(choice.player, p2.id);
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let listed = choice.candidates(&answers);
                let legal: BTreeSet<Entity> = listed
                    .iter()
                    .filter(|(_, legality)| *legality == rules::Legality::Legal)
                    .map(|(entity, _)| *entity)
                    .collect();
                assert_eq!(legal, BTreeSet::from([Entity::from(p1), Entity::from(p3)]));
                assert!(
                    listed.iter().any(|(entity, legality)| *entity == Entity::from(p2)
                        && matches!(
                            legality,
                            rules::Legality::Illegal(reasons)
                            if reasons.contains(&rules::Why::NotMatching)
                        ))
                );
            })
            .unwrap();
        game.play([p2.choose_defending_player(p3)])?;
        game.play([
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.pass_to(turns::Step::DeclareAttackers),
            p3.pass_to(turns::Step::DeclareAttackers),
            p2.declare_attackers([bears]),
        ])?;
        game.play([
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_to(turns::Step::CombatDamage),
            p3.pass_to(turns::Step::CombatDamage),
        ])?;
        game.play([p2.pass_priority(), p3.pass_priority(), p1.pass_priority()])?;

        assert_eq!(game.life(p3), 18);
        assert_eq!(game.life(p1), 20);
        Ok(())
    }

    #[test]
    fn a_player_who_has_lost_cannot_be_chosen_as_the_defending_player() -> TestResult {
        let (mut game, [p1, p2, p3, p4]) = MockGame::new([P1, P2, P3, P4.life(0)]);
        game.act(p1, actions::Action::Choose(vec![p4.into()]));
        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NothingToChoose { player: p1.id })
        );
        game.play([
            p1.pass_to(turns::Step::BeginningOfCombat),
            p2.pass_to(turns::Step::BeginningOfCombat),
            p3.pass_to(turns::Step::BeginningOfCombat),
            p4.pass_to(turns::Step::BeginningOfCombat),
        ])?;
        assert!(game.has_lost(p4));
        let actions::Choice::Pick(choice) = game
            .app
            .world()
            .get_resource::<actions::PendingChoice>()
            .and_then(|pending| (**pending).clone())
            .expect("a pending defending-player choice")
        else {
            panic!("expected a pick choice");
        };
        assert_eq!(choice.id, DEFENDING_PLAYER);
        assert_eq!(choice.player, p1.id);
        game.app
            .world_mut()
            .run_system_once(move |answers: rules::Rules| {
                let listed = choice.candidates(&answers);
                let legal: BTreeSet<Entity> = listed
                    .iter()
                    .filter(|(_, legality)| *legality == rules::Legality::Legal)
                    .map(|(entity, _)| *entity)
                    .collect();
                assert_eq!(legal, BTreeSet::from([Entity::from(p2), Entity::from(p3)]));
                assert!(
                    listed.iter().any(|(entity, legality)| *entity == Entity::from(p1)
                        && matches!(
                            legality,
                            rules::Legality::Illegal(reasons)
                            if reasons.contains(&rules::Why::NotMatching)
                        ))
                );
                assert!(listed.iter().all(|(entity, _)| *entity != Entity::from(p4)));
            })
            .unwrap();

        game.act(p1, actions::Action::Choose(vec![p4.into()]));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::InvalidChoice {
                player: p1.id,
                why: actions::ChoiceWhy::NotAllowed(p4.into()),
            })
        );
        assert!(!game.has::<players::DefendingPlayer>(p4));
        Ok(())
    }
}
