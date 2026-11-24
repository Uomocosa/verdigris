use crate::events;
use bevy::prelude::{MessageReader, MessageWriter};

use crate::Attempt;
use crate::players;
use crate::turns;

/// 103.6. Some cards allow a player to take actions with them from their opening hand. Once the
/// mulligan process (see rule 103.5) is complete, the starting player may take any such actions in
/// any order. Then each other player in turn order may do the same.
///
/// # Notes
///
/// All `Attempt`s are announced at once, so a mandatory effect of a later player would happen before
/// an earlier player answers.
pub fn r103_6_opening_hand_actions(
    mut mulligans_complete: MessageReader<turns::MulligansComplete>,
    mut opening_hand_actions: MessageWriter<Attempt<events::OpeningHandAction>>,
    players: players::Players,
) {
    for _ in mulligans_complete.read() {
        for player in players.still_in_the_game() {
            for card in player.opening_hand() {
                opening_hand_actions.write(Attempt(events::OpeningHandAction {
                    card,
                    player: player.id,
                }));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::cards;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, HAND};

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
        assert!(game.player(p1).cards_in(HAND).has(bolt));
        assert!(!game.stack().has(bolt));
        Ok(())
    }

    #[test]
    fn playing_a_land_while_an_opening_hand_action_is_waiting_is_rejected() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([LEYLINE_OF_ANTICIPATION, FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;

        game.act(p1, actions::Action::PlayLand(forest.into()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotPlayLand {
                land: forest.into()
            })
        );
        assert!(game.player(p1).cards_in(HAND).has(forest));
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
                attackers: vec![bears.into()]
            })
        );
        assert!(!game.is_attacking(bears));
        Ok(())
    }

    #[test]
    fn opening_hand_choices_follow_turn_order() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([
            P1.hand([LEYLINE_OF_ANTICIPATION]),
            P2.hand([LEYLINE_OF_ANTICIPATION]),
            P3.hand([LEYLINE_OF_ANTICIPATION]),
        ]);
        let [p1_leyline] = game.player(p1).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;
        let [p2_leyline] = game.player(p2).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;
        let [p3_leyline] = game.player(p3).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;

        game.act(p3, actions::Action::ChooseMay(vec![p3_leyline.into()]));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::InvalidMay { player: p3.id })
        );
        assert!(game.player(p3).cards_in(HAND).has(p3_leyline));

        game.act(p2, actions::Action::ChooseMay(vec![p2_leyline.into()]));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::InvalidMay { player: p2.id })
        );
        assert!(game.player(p2).cards_in(HAND).has(p2_leyline));

        game.act(p1, actions::Action::ChooseMay(vec![p1_leyline.into()]));

        assert!(game.player(p1).cards_in(BATTLEFIELD).has(p1_leyline));
        assert_eq!(game.priority().holder, None);

        game.act(p1, actions::Action::PassPriority);

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NoPriority { player: p1.id })
        );
        assert_eq!(game.priority().holder, None);

        game.act(p2, actions::Action::ChooseMay(vec![p2_leyline.into()]));

        assert!(game.player(p2).cards_in(BATTLEFIELD).has(p2_leyline));
        assert_eq!(game.priority().holder, None);

        game.act(p3, actions::Action::ChooseMay(vec![p3_leyline.into()]));

        assert!(game.player(p1).cards_in(BATTLEFIELD).has(p1_leyline));
        assert!(game.player(p2).cards_in(BATTLEFIELD).has(p2_leyline));
        assert!(game.player(p3).cards_in(BATTLEFIELD).has(p3_leyline));
        assert_eq!(game.priority_player(), Some(p1.id));
        Ok(())
    }
}
