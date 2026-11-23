use bevy::prelude::{Commands, MessageReader, MessageWriter, ParamSet, ResMut};

use crate::StateBasedActionsPerformed;
use crate::players;
use crate::rules;
use crate::rules::traits::event::Event;

type WinMessages<'reader_w, 'reader_s, 'writer> = (
    MessageReader<'reader_w, 'reader_s, rules::Would<rules::WinGame>>,
    MessageWriter<'writer, rules::Would<rules::WinGame>>,
);

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
pub fn r614_1a_r616_1_replacement_effects(
    mut attempted: MessageReader<rules::Would<rules::DrawFromEmptyLibrary>>,
    mut losses: MessageReader<rules::Would<rules::LoseGame>>,
    mut wins: ParamSet<WinMessages<'_, '_, '_>>,
    mut commands: Commands,
    mut state_based_actions_performed: ResMut<StateBasedActionsPerformed>,
    answers: rules::Rules,
) {
    for would in attempted.read() {
        let rules::Would(event) = would;
        match answers.replacement(**event, rules::DrawFromEmptyLibrary::ID) {
            Some(rules::Outcome::WinTheGame) => {
                wins.p1().write(rules::Would(rules::WinGame(**event)));
                **state_based_actions_performed = true;
            }
            None => {
                commands.entity(**event).insert(players::AttemptedDrawFromEmptyLibrary);
                **state_based_actions_performed = true;
            }
        }
    }
    for would in losses.read() {
        let rules::Would(event) = would;
        if answers.legal::<rules::LosesTheGame>(&event.context()) {
            commands.entity(**event).insert(players::Lost);
            **state_based_actions_performed = true;
        }
    }
    for would in wins.p0().read() {
        let rules::Would(event) = would;
        if answers.legal::<rules::WinsTheGame>(&event.context()) {
            commands.entity(**event).insert(players::Won);
            **state_based_actions_performed = true;
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
