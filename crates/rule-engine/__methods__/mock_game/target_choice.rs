use bevy::ecs::system::RunSystemOnce;
use std::collections::BTreeSet;

use crate::TARGET;
use crate::actions;
use crate::rules;
use crate::testing;

pub fn target_choice(game: &mut testing::MockGame) -> Option<testing::ExpectedTargets> {
    let pending = game.app.world().get_resource::<actions::PendingChoice>()?.as_ref()?.clone();
    let actions::Choice::Pick(choice) = pending else {
        return None;
    };
    if choice.id != TARGET {
        return None;
    }
    let legal = match &choice.among {
        actions::Candidates::Fixed(set) => set.clone(),
        actions::Candidates::Matching { .. } => {
            let matching = choice.clone();
            game.app
                .world_mut()
                .run_system_once(move |answers: rules::Rules| {
                    matching
                        .candidates(&answers)
                        .into_iter()
                        .filter(|(_, legality)| *legality == rules::Legality::Legal)
                        .map(|(entity, _)| entity)
                        .collect::<BTreeSet<_>>()
                })
                .unwrap_or_default()
        }
    };
    Some(testing::ExpectedTargets {
        count: choice.count.max,
        legal,
    })
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::target_choice;

    #[test]
    fn test_usage() {
        let (mut game, _) = MockGame::new([P1, P2]);
        assert!(target_choice(&mut game).is_none());
    }
}
