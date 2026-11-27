use crate::actions;
use crate::rules;
use crate::testing;

pub fn check_pick_previews(game: &mut testing::MockGame, disagreements: &mut Vec<String>) {
    let Some(actions::Choice::Pick(choice)) =
        (**game.app.world().resource::<actions::PendingChoice>()).clone()
    else {
        return;
    };
    if choice.count.min != 1 || choice.count.max != 1 || choice.ordered {
        return;
    }
    let legal = game.app.world().resource::<actions::LegalActions>().clone();
    for (action, legality) in &legal.actions {
        let actions::Action::Choose(picked) = action else {
            continue;
        };
        let accepted = game.pick_preview_accepts(&choice, picked);
        let listed = *legality == rules::Legality::Legal;
        if listed && !accepted {
            disagreements.push(format!(
                "{action:?} is listed Legal but the engine rejects it"
            ));
        }
        if !listed && accepted {
            disagreements.push(format!(
                "{action:?} is listed Illegal but the engine accepts it"
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::testing::*;

    use super::check_pick_previews;

    #[test]
    fn test_usage() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [_forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;
        game.app.update();
        let mut disagreements: Vec<String> = Vec::new();
        check_pick_previews(&mut game, &mut disagreements);
        assert_eq!(disagreements, Vec::<String>::new());
        assert!(game.player(p1).cards_in(HAND).has(FOREST));
        Ok(())
    }
}
