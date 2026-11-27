use bevy::prelude::Entity;

use crate::actions;
use crate::cards;
use crate::rules;
use crate::testing;
use crate::zones;

pub fn assert_legal_actions_agree(game: &mut testing::MockGame) -> testing::TestResult {
    let legal = game.app.world().resource::<actions::LegalActions>().clone();
    let Some(holder) = legal.player else {
        return Ok(());
    };
    let mut candidates: Vec<(actions::Action, Vec<Entity>, bool)> = Vec::new();
    for (action, legality) in &legal.actions {
        match action {
            actions::Action::Play(card) | actions::Action::Cast(card) => candidates.push((
                action.clone(),
                vec![*card],
                *legality == rules::Legality::Legal,
            )),
            actions::Action::ActivateManaAbility { .. }
            | actions::Action::PassPriority
            | actions::Action::Concede
            | actions::Action::Choose(_)
            | actions::Action::AssignCombatDamage(_)
            | actions::Action::DeclareAttackers(_)
            | actions::Action::DeclareBlockers(_) => {}
        }
    }
    for (creature, legality) in &legal.attackers {
        candidates.push((
            actions::Action::DeclareAttackers(vec![*creature]),
            vec![*creature],
            *legality == rules::Legality::Legal,
        ));
    }
    for (assignment, legality) in &legal.blocks {
        candidates.push((
            actions::Action::DeclareBlockers(vec![*assignment]),
            vec![assignment.blocker, assignment.attacker],
            *legality == rules::Legality::Legal,
        ));
    }
    let mut disagreements: Vec<String> = Vec::new();
    for (action, involved, listed) in candidates {
        let engine = game.oracle_engine_accepts(holder, &action);
        if listed && !engine {
            disagreements.push(format!(
                "{action:?} is listed Legal but the engine rejects it"
            ));
            continue;
        }
        if !listed && engine {
            disagreements.push(format!(
                "{action:?} is listed Illegal but the engine accepts it"
            ));
            continue;
        }
        if listed {
            continue;
        }
        let safe = matches!(
            &action,
            actions::Action::DeclareAttackers(_)
                | actions::Action::DeclareBlockers(_)
                | actions::Action::Play(_)
                | actions::Action::Cast(_)
        );
        if !safe {
            continue;
        }
        let snapshot_of = |game: &testing::MockGame| {
            involved
                .iter()
                .map(|card| {
                    (
                        game.app.world().get::<zones::InZone>(*card).copied(),
                        game.app.world().get::<cards::Controller>(*card).is_some(),
                    )
                })
                .collect::<Vec<_>>()
        };
        let before = snapshot_of(game);
        let rejections_before = game.rejection_count();
        game.act(*holder, action.clone());
        if game.rejection_count() == rejections_before {
            disagreements.push(format!(
                "{action:?} is listed Illegal but the engine accepts it"
            ));
        }
        if snapshot_of(game) != before {
            disagreements.push(format!(
                "{action:?} is listed Illegal but trying it changes state"
            ));
        }
    }
    if disagreements.is_empty() {
        Ok(())
    } else {
        Err(testing::TestError::OracleDisagreement(
            disagreements.join("\n"),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;

    use super::assert_legal_actions_agree;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [_forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;
        game.app.update();

        assert_legal_actions_agree(&mut game)?;

        assert!(game.player(p1).cards_in(HAND).has(FOREST));
        Ok(())
    }
}
