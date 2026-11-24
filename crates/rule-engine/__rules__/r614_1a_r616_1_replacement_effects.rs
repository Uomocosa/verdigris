use bevy::prelude::{Commands, MessageReader, MessageWriter, ResMut};

use crate::StateBasedActionsPerformed;
use crate::rules;
use crate::rules::traits::event::Event;

/// 614.1a Effects that use the word “instead” are replacement effects. Most replacement effects use the word “instead” to indicate what events will be replaced with other events.
///
/// 616.1. If two or more replacement and/or prevention effects are attempting to modify the way an
/// event affects an object or player, the affected object’s controller (or its owner if it has no
/// controller) or the affected player chooses one to apply, following the steps listed below. If
/// two or more players have to make these choices at the same time, choices are made in APNAP
/// order (see rule 101.4).
///
/// # Notes
///
/// When several replacements could apply to the same event, CR 616.1 lets the affected player
/// choose one. This system applies the earliest-timestamped `Instead` instead; player choice
/// between several replacements is out of scope.
pub fn r614_1a_r616_1_replacement_effects<E: Event>(
    mut incoming: MessageReader<rules::Would<E>>,
    mut proceeds: MessageWriter<rules::Proceeds<E>>,
    mut asks: MessageWriter<rules::Ask<E>>,
    mut commands: Commands,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
    answers: rules::Rules,
) {
    for would in incoming.read() {
        let event = &**would;
        if !answers.legal(event) {
            continue;
        }
        match answers.replacement(event) {
            Some(rules::Then::Events(then)) => {
                for template in then {
                    template.instantiate(event).announce(&mut commands);
                }
                **state_based_actions_performed = true;
            }
            Some(rules::Then::May(then)) => {
                asks.write(rules::Ask {
                    event: event.clone(),
                    then,
                });
            }
            None => {
                proceeds.write(rules::Proceeds(event.clone()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn opponent_cannot_win_while_platinum_angel_is_on_the_battlefield() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::builder().turn(1, turns::Step::Untap).players([
            P1.battlefield([FELIDAR_SOVEREIGN]).life(40),
            P2.battlefield([PLATINUM_ANGEL]),
        ]);
        game.play([p1.pass_priority(), p1.pass_priority(), p2.pass_priority()])?;
        assert!(!game.has_won(p1));
        assert!(!game.has_lost(p1));
        Ok(())
    }

    #[test]
    fn drawing_from_an_empty_library_with_laboratory_maniac_wins_the_game_instead() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DIVINATION])
                .battlefield([LABORATORY_MANIAC])
                .empty_library()
                .mana_pool(mana![2, U]),
            P2,
        ]);
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION)?;
        game.play([p1.cast(divination).pass_priority(), p2.pass_priority()])?;
        assert!(game.has_won(p1));
        assert!(!game.has_lost(p1));
        Ok(())
    }

    #[test]
    fn platinum_angel_stops_the_laboratory_maniac_win_without_losing() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([DIVINATION])
                .battlefield([LABORATORY_MANIAC])
                .empty_library()
                .mana_pool(mana![2, U]),
            P2.battlefield([PLATINUM_ANGEL]),
        ]);
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION)?;
        game.play([p1.cast(divination).pass_priority(), p2.pass_priority()])?;
        assert!(!game.has_won(p1));
        assert!(!game.has_lost(p1));
        Ok(())
    }
}
