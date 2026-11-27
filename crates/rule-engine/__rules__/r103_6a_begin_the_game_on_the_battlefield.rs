use crate::events;
use bevy::prelude::{Commands, MessageReader, MessageWriter};

use crate::Attempt;
use crate::Resolve;
use crate::cards;

/// 103.6a If a card allows a player to begin the game with that card on the battlefield, the player
/// taking this action puts that card onto the battlefield.
pub fn r103_6a_begin_the_game_on_the_battlefield(
    mut beginning: MessageReader<Resolve<events::BeginTheGameOnTheBattlefield>>,
    mut entering: MessageWriter<Attempt<events::EnterTheBattlefield>>,
    mut commands: Commands,
) {
    for action in beginning.read() {
        commands.entity(action.card).insert(cards::Controller(action.player));
        entering.write(Attempt(events::EnterTheBattlefield {
            permanent: action.card,
        }));
    }
}

#[cfg(test)]
mod tests {
    use crate::actions;
    use crate::cards;
    use crate::testing::*;
    use crate::turns;
    use crate::{BATTLEFIELD, HAND};

    #[test]
    fn leyline_in_the_opening_hand_is_put_onto_the_battlefield() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([
            P1.hand([LEYLINE_OF_ANTICIPATION, DIVINATION]).battlefield([TUNDRA, TUNDRA, TUNDRA]),
            P2,
        ]);
        let [leyline] = game.player(p1).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;

        game.act(p1, actions::Action::ChooseMay(vec![leyline.into()]));

        assert!(game.moved(leyline).to(BATTLEFIELD).of(p1));
        assert!(game.player(p1).cards_in(HAND).has_no(LEYLINE_OF_ANTICIPATION));
        assert_eq!(game.turn().number, 1);
        assert_eq!(game.priority_player(), Some(p1.id));
        assert!(game.last_rejection().is_none());
        Ok(())
    }

    #[test]
    fn leyline_on_the_battlefield_from_the_opening_hand_grants_flash() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::builder().turn(2, turns::Step::PrecombatMain).players([
                P1.hand([LEYLINE_OF_ANTICIPATION, DIVINATION]).battlefield([
                    RELIQUARY_TOWER,
                    RELIQUARY_TOWER,
                    TUNDRA,
                ]),
                P2.active(),
            ]);
        let [leyline] = game.player(p1).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;
        let [tower_1, tower_2] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(RELIQUARY_TOWER)?;
        let [tundra] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TUNDRA)?;
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION)?;

        game.act(p1, actions::Action::ChooseMay(vec![leyline.into()]));
        assert!(game.moved(leyline).to(BATTLEFIELD).of(p1));

        game.play([
            p2.pass_priority(),
            p1.activate_ability(tower_1.mana_ability(cards::ManaType::Colorless)),
            p1.activate_ability(tower_2.mana_ability(cards::ManaType::Colorless)),
            p1.activate_ability(tundra.mana_ability(cards::ManaType::from(cards::Color::Blue))),
            p1.cast(divination),
        ])?;

        assert!(game.stack().has(DIVINATION));
        assert_eq!(game.mana_pool(p1), cards::ManaPool::EMPTY);
        Ok(())
    }

    #[test]
    fn passing_the_opening_hand_choice_leaves_leyline_in_hand() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([LEYLINE_OF_ANTICIPATION]), P2]);
        let [leyline] = game.player(p1).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;

        game.act(p1, actions::Action::ChooseMay(Vec::new()));

        assert!(game.card(leyline).is_in(HAND).of(p1));
        assert!(game.player(p1).cards_in(BATTLEFIELD).has_no(LEYLINE_OF_ANTICIPATION));
        assert!(game.last_rejection().is_none());
        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.turn().step, turns::Step::BeginningOfCombat);
        Ok(())
    }
}
