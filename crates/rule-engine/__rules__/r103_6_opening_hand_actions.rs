use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::{ResMut, on_message};

use crate::events;
use crate::players;
use crate::rules;
use crate::turns;

/// 103.6. Some cards allow a player to take actions with them from their opening hand. Once the
/// mulligan process (see rule 103.5) is complete, the starting player may take any such actions in
/// any order. Then each other player in turn order may do the same.
pub fn r103_6_opening_hand_actions() -> ScheduleConfigs<ScheduleSystem> {
    opening_hand_actions.run_if(on_message::<turns::MulligansComplete>)
}

fn opening_hand_actions(
    mut hooks: ResMut<rules::HookQueue<events::OpeningHandAction>>,
    players: players::Players,
) {
    let events_in_turn_order: Vec<events::OpeningHandAction> = players
        .still_in_the_game()
        .iter()
        .flat_map(players::PlayerView::opening_hand)
        .map(|card| events::OpeningHandAction { card })
        .collect();
    hooks.announce(events_in_turn_order);
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::cards;
    use crate::mana;
    use crate::rules;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, GRAVEYARD, HAND};

    #[test]
    fn a_card_that_left_the_opening_hand_takes_no_action() -> TestResult {
        let (mut game, [p1, ..]) =
            MockGame::new([P1.hand([TEST_OPENING_PURGE, LEYLINE_OF_ANTICIPATION]), P2]);
        let [purge] = game.player(p1).cards_in(HAND).exactly::<1>(TEST_OPENING_PURGE)?;
        let [leyline] = game.player(p1).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;

        game.act(p1, actions::Action::PassPriority);

        assert!(game.moved(purge).to(GRAVEYARD).of(p1));
        assert!(game.moved(leyline).to(GRAVEYARD).of(p1));

        game.act(p1, actions::Action::Choose(vec![leyline.into()]));

        assert!(game.moved(leyline).to(GRAVEYARD).of(p1));
        assert!(!game.card(leyline).is_in(BATTLEFIELD).of(p1));
        Ok(())
    }

    #[test]
    fn opening_hand_actions_happen_in_the_chosen_order() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([TEST_MAY_PURGE, LEYLINE_OF_ANTICIPATION]), P2]);
        let [purge] = game.player(p1).cards_in(HAND).exactly::<1>(TEST_MAY_PURGE)?;
        let [leyline] = game.player(p1).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;

        game.act(
            p1,
            actions::Action::Choose(vec![purge.into(), leyline.into()]),
        );

        assert_eq!(game.priority().holder, None);
        assert!(game.moved(purge).to(GRAVEYARD).of(p1));
        assert!(game.moved(leyline).to(GRAVEYARD).of(p1));

        game.act(p2, actions::Action::PassPriority);

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NoPriority { player: p2.id })
        );
        assert!(game.moved(purge).to(GRAVEYARD).of(p1));
        assert!(game.moved(leyline).to(GRAVEYARD).of(p1));
        assert!(!game.card(leyline).is_in(BATTLEFIELD).of(p1));
        let [purge_buried] = game.player(p1).cards_in(GRAVEYARD).exactly::<1>(TEST_MAY_PURGE)?;
        let [leyline_buried] =
            game.player(p1).cards_in(GRAVEYARD).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;
        game.play([p1.arrange_graveyard([purge_buried, leyline_buried])])?;
        assert_eq!(game.priority_player(), Some(p1.id));

        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([TEST_MAY_PURGE, LEYLINE_OF_ANTICIPATION]), P2]);
        let [purge] = game.player(p1).cards_in(HAND).exactly::<1>(TEST_MAY_PURGE)?;
        let [leyline] = game.player(p1).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;

        game.act(
            p1,
            actions::Action::Choose(vec![leyline.into(), purge.into()]),
        );

        assert_eq!(game.priority().holder, None);
        assert!(game.moved(leyline).to(BATTLEFIELD).of(p1));

        game.act(p2, actions::Action::PassPriority);

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NoPriority { player: p2.id })
        );
        assert!(game.moved(leyline).to(BATTLEFIELD).of(p1));
        assert!(game.moved(purge).to(GRAVEYARD).of(p1));
        assert_eq!(game.priority_player(), Some(p1.id));
        Ok(())
    }

    #[test]
    fn a_later_player_waits_for_the_choice_of_an_earlier_players_action() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([TEST_OPENING_TOLL, GRIZZLY_BEARS]),
            P2.hand([TEST_DOUBLE_GIFT]),
        ]);
        let [bears] = game.player(p1).cards_in(HAND).exactly::<1>(GRIZZLY_BEARS)?;

        game.act(p2, actions::Action::PassPriority);

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NoPriority { player: p2.id })
        );
        assert_eq!(game.life(p2), 20);

        game.act(p1, actions::Action::Choose(vec![bears.into()]));

        assert!(game.moved(bears).to(GRAVEYARD).of(p1));
        assert_eq!(game.life(p2), 20);

        game.act(p2, actions::Action::PassPriority);

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NoPriority { player: p2.id })
        );
        assert_eq!(game.life(p2), 23);
        assert_eq!(game.priority_player(), Some(p1.id));
        Ok(())
    }

    #[test]
    fn every_opening_hand_action_of_a_card_is_taken() {
        let (mut game, [p1, _]) = MockGame::new([P1.hand([TEST_DOUBLE_GIFT]), P2]);
        game.pass_priority();
        assert_eq!(game.life(p1), 23);
    }

    #[test]
    fn casting_while_an_opening_hand_action_is_waiting_is_rejected() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([
            P1.hand([LEYLINE_OF_ANTICIPATION, LIGHTNING_BOLT]).mana_pool(mana![R]),
            P2,
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;

        game.act(p1, actions::Action::Cast(bolt.into()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotCast { spell: bolt.into() })
        );
        assert!(game.card(bolt).is_in(HAND).of(p1));
        assert!(!game.card(bolt).is_on_the_stack());
        Ok(())
    }

    #[test]
    fn playing_a_land_while_an_opening_hand_action_is_waiting_is_rejected() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([LEYLINE_OF_ANTICIPATION, FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;

        game.act(p1, actions::Action::Play(forest.into()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotPlay {
                card: forest.into()
            })
        );
        assert!(game.card(forest).is_in(HAND).of(p1));
        Ok(())
    }

    #[test]
    fn activating_a_mana_ability_while_an_opening_hand_action_is_waiting_is_rejected() -> TestResult
    {
        let (mut game, [p1, ..]) =
            MockGame::new([P1.hand([LEYLINE_OF_ANTICIPATION]).battlefield([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(FOREST)?;

        game.act(
            p1,
            actions::Action::ActivateManaAbility {
                source: forest.into(),
                produces: cards::ManaType::from(cards::Color::Green),
            },
        );

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotActivateManaAbility {
                permanent: forest.into()
            })
        );
        assert_eq!(game.mana_pool(p1), mana![]);
        assert!(!game.is_tapped(forest));
        Ok(())
    }

    #[test]
    fn declaring_attackers_while_an_opening_hand_action_is_waiting_is_rejected() -> TestResult {
        let (mut game, [p1, ..]) =
            MockGame::builder().turn(1, turns::Step::DeclareAttackers).players([
                P1.hand([LEYLINE_OF_ANTICIPATION]).battlefield([GRIZZLY_BEARS]),
                P2,
            ]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS)?;

        game.act(p1, actions::Action::DeclareAttackers(vec![bears.into()]));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotAttack {
                attackers: vec![bears.into()],
                why: vec![actions::DeclarationWhy {
                    creature: None,
                    why: rules::Why::WrongTiming,
                }],
            })
        );
        assert!(!game.is_attacking(bears));
        Ok(())
    }

    #[test]
    fn opening_hand_choices_follow_turn_order() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([
            P1.hand([LEYLINE_OF_ANTICIPATION]),
            P2.hand([LEYLINE_OF_ANTICIPATION, TEST_DOUBLE_GIFT]),
            P3.hand([LEYLINE_OF_ANTICIPATION]),
        ]);
        let [p1_leyline] = game.player(p1).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;
        let [p2_leyline] = game.player(p2).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;
        let [p3_leyline] = game.player(p3).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;

        game.act(p3, actions::Action::Choose(vec![p3_leyline.into()]));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::InvalidChoice {
                player: p3.id,
                why: actions::ChoiceWhy::NotAllowed(p3_leyline.into()),
            })
        );
        assert!(game.card(p3_leyline).is_in(HAND).of(p3));

        game.act(p2, actions::Action::Choose(vec![p2_leyline.into()]));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::InvalidChoice {
                player: p2.id,
                why: actions::ChoiceWhy::NotAllowed(p2_leyline.into()),
            })
        );
        assert!(game.card(p2_leyline).is_in(HAND).of(p2));

        assert_eq!(game.life(p2), 20);

        game.act(p1, actions::Action::Choose(vec![p1_leyline.into()]));

        assert!(game.moved(p1_leyline).to(BATTLEFIELD).of(p1));
        assert_eq!(game.priority().holder, None);

        game.act(p1, actions::Action::PassPriority);

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NoPriority { player: p1.id })
        );
        assert_eq!(game.priority().holder, None);
        assert_eq!(game.life(p2), 23);

        game.act(p2, actions::Action::Choose(vec![p2_leyline.into()]));

        assert!(game.moved(p2_leyline).to(BATTLEFIELD).of(p2));
        assert_eq!(game.priority().holder, None);

        game.act(p3, actions::Action::Choose(vec![p3_leyline.into()]));

        assert!(game.moved(p1_leyline).to(BATTLEFIELD).of(p1));
        assert!(game.moved(p2_leyline).to(BATTLEFIELD).of(p2));
        assert!(game.moved(p3_leyline).to(BATTLEFIELD).of(p3));
        assert_eq!(game.priority_player(), Some(p1.id));
        Ok(())
    }
}
