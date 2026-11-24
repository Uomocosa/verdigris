use crate::STACK;
use bevy::prelude::{MessageWriter, Query};

use crate::GameMut;
use crate::stack;
use crate::turns;
use crate::zones;

/// 117.4. If all players pass in succession (that is, if all players pass without taking any
/// actions in between passing), the spell or ability on top of the stack resolves or, if the stack
/// is empty, the phase or step ends.
pub fn r117_4_priority(
    mut resolutions: MessageWriter<stack::Resolution>,
    mut all_players_passed: MessageWriter<turns::AllPlayersPassed>,
    mut game: GameMut,
    cards: Query<(&zones::InZone, Option<&stack::CastingStage>)>,
    zones: zones::ZoneLookup,
    stack_contents: Query<&zones::ZoneContents>,
) {
    if !game.have_all_players_passed_in_succession() {
        return;
    }
    let holder = game.priority_holder();
    game.give_priority_to(holder.id);
    let top = zones
        .find(STACK, None)
        .and_then(|stack| stack_contents.get(stack).ok())
        .and_then(|contents| contents.last().copied());
    let Some(top) = top else {
        all_players_passed.write(turns::AllPlayersPassed);
        return;
    };
    let is_still_casting = cards.get(top).is_ok_and(|(_, stage)| stage.is_some());
    if is_still_casting {
        return;
    }
    game.give_priority_to_active_player();
    resolutions.write(stack::Resolution(top));
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{GRAVEYARD, HAND};

    #[test]
    fn all_players_passing_resolves_the_spell() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;
        assert!(game.player(p1).cards_in(GRAVEYARD).has(bolt));
        Ok(())
    }

    #[test]
    fn all_players_passing_resolves_only_the_top_spell() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, LIGHTNING_BOLT]).mana_pool(mana![R, R]),
            P2,
        ]);
        let [bolt_1, bolt_2] = game.player(p1).cards_in(HAND).exactly::<2>(LIGHTNING_BOLT)?;
        game.play([
            p1.cast(bolt_1).targeting(p2),
            p1.cast(bolt_2).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])?;

        assert!(game.player(p1).cards_in(GRAVEYARD).has(bolt_2));
        assert!(game.stack().has(bolt_1));
        assert_eq!(game.life(p2), 17);
        Ok(())
    }

    #[test]
    fn two_rounds_of_passing_resolve_both_spells() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, LIGHTNING_BOLT]).mana_pool(mana![R, R]),
            P2,
        ]);
        let [bolt_1, bolt_2] = game.player(p1).cards_in(HAND).exactly::<2>(LIGHTNING_BOLT)?;
        game.play([
            p1.cast(bolt_1).targeting(p2),
            p1.cast(bolt_2).targeting(p2).pass_priority(),
            p2.pass_priority(),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert!(game.player(p1).cards_in(GRAVEYARD).has(bolt_2));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(bolt_1));
        assert_eq!(game.life(p2), 14);
        Ok(())
    }

    #[test]
    fn priority_returns_to_the_active_player_after_a_spell_resolves() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.cast(bolt).targeting(p2).pass_priority()])?;

        assert_ne!(game.priority_player(), p1.id);
        game.play([p2.pass_priority()])?;

        let priority = game.priority();
        assert_eq!(priority.holder, p1.id);
        assert!(priority.passed.is_empty());
        Ok(())
    }

    #[test]
    fn the_step_ends_when_every_player_still_in_the_game_has_passed() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2, P3]);

        game.act(p3, actions::Action::Concede);
        game.play([p1.pass_priority(), p2.pass_priority()])?;

        assert_eq!(game.turn().step, turns::Step::BeginningOfCombat);
        Ok(())
    }

    #[test]
    fn a_player_who_passed_and_then_left_does_not_count_for_the_others() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([P1, P2, P3]);

        game.play([p1.pass_priority()])?;
        game.act(p1, actions::Action::Concede);
        game.play([p2.pass_priority()])?;

        assert_eq!(game.turn().step, turns::Step::PrecombatMain);
        assert_eq!(game.priority_player(), p3.id);
        Ok(())
    }
}
