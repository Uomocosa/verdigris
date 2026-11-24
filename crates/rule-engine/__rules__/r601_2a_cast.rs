use crate::STACK;
use bevy::prelude::{Commands, MessageReader, MessageWriter};

use crate::actions;
use crate::events;
use crate::rules;
use crate::zones;

/// 601.2a To propose the casting of a spell, a player first moves that card (or that copy of a
/// card) from where it is to the stack. It becomes the topmost object on the stack. It has all the
/// characteristics of the card (or the copy of a card) associated with it, and that player becomes
/// its controller. Any continuous effects that modify the characteristics of the spell as you start
/// casting it begin as it is put on the stack (see rule 611.2f). The spell remains on the stack
/// until it resolves, it’s countered, or a rule or effect moves it elsewhere.
///
/// # See also
///
/// 305.9. If an object is both a land and another card type, it can be played only as a land. It
/// can’t be cast as a spell.
///
/// 300.2a An object that’s both a land and another card type (for example, an artifact land) can
/// only be played as a land. It can’t be cast as a spell.
///
/// 202.1b Some objects have no mana cost. This normally includes all land cards, any other cards
/// that have no mana symbols where their mana cost would appear, tokens (unless the effect that
/// creates them specifies otherwise), and nontraditional Magic cards. Having no mana cost
/// represents an unpayable cost (see rule 118.6). Note that lands are played without paying any
/// costs (see rule 305, “Lands”).
///
/// 118.6. Some objects have no mana cost. This represents an unpayable cost. An ability can also
/// have an unpayable cost if its cost is based on the mana cost of an object with no mana cost.
/// Attempting to cast a spell or activate an ability that has an unpayable cost is a legal
/// action. However, attempting to pay an unpayable cost is an illegal action.
pub fn r601_2a_cast(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut commands: Commands,
    answers: rules::Rules,
) {
    for action in actions.read() {
        let actions::Action::Cast(spell) = &action.action else {
            continue;
        };
        let spell = *spell;
        if !answers.legal::<events::CastSpell>(&events::CastSpell {
            spell,
            player: action.player,
        }) {
            rejected.write(actions::Rejected::CannotCast { spell });
            continue;
        }
        let Some(stack) = answers.zones.find(STACK, None) else {
            rejected.write(actions::Rejected::CannotCast { spell });
            continue;
        };
        commands.entity(spell).insert(zones::InZone(stack));
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::actions;
    use crate::mana;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn casting_an_instant_moves_it_to_the_stack() -> TestResult {
        let (mut game, [p1, ..]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        game.play([p1.cast(bolt)])?;
        assert!(game.stack().has(bolt));
        Ok(())
    }

    #[test]
    fn sorcery_cannot_be_cast_on_an_opponents_turn() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::builder()
            .turn(2, turns::Step::PrecombatMain)
            .players([P1.hand([DIVINATION]).mana_pool(mana![2, U]), P2.active()]);
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION)?;

        game.play([p2.pass_priority()])?;
        let err = game.play([p1.cast(divination)]).unwrap_err();
        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::CannotCast {
                spell: divination.into()
            })
        );
        assert!(game.player(p1).cards_in(HAND).has(divination));
        Ok(())
    }

    #[test]
    fn sorcery_cannot_be_cast_while_the_stack_is_not_empty() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, DIVINATION]).mana_pool(mana![R, 2, U]),
            P2,
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION)?;

        game.play([p1.cast(bolt).targeting(p2)])?;
        let err = game.play([p1.cast(divination)]).unwrap_err();
        assert_eq!(
            err.rejection(),
            Some(&actions::Rejected::CannotCast {
                spell: divination.into()
            })
        );
        assert!(game.player(p1).cards_in(HAND).has(divination));
        Ok(())
    }

    #[test]
    fn a_land_cannot_be_cast_as_a_spell() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;

        game.act(p1, actions::Action::Cast(forest.into()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotCast {
                spell: forest.into()
            })
        );
        assert!(game.player(p1).cards_in(HAND).has(forest));
        assert!(!game.stack().has(forest));
        Ok(())
    }

    #[test]
    fn a_sorcery_cannot_be_cast_during_combat() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::builder()
            .turn(1, turns::Step::DeclareAttackers)
            .players([P1.hand([DIVINATION]).mana_pool(mana![2, U]), P2]);
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION)?;

        game.act(p1, actions::Action::Cast(divination.into()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotCast {
                spell: divination.into()
            })
        );
        assert!(game.player(p1).cards_in(HAND).has(divination));
        assert!(!game.stack().has(divination));
        Ok(())
    }

    #[test]
    fn leyline_of_anticipation_lets_a_player_cast_a_sorcery_on_an_opponents_turn() -> TestResult {
        let (mut game, [p1, p2]) =
            MockGame::builder().turn(2, turns::Step::PrecombatMain).players([
                P1.battlefield([LEYLINE_OF_ANTICIPATION]).hand([DIVINATION]).mana_pool(mana![2, U]),
                P2.active(),
            ]);
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION)?;

        game.play([p2.pass_priority()])?;
        game.play([p1.cast(divination)])?;

        assert!(game.stack().has(divination));
        Ok(())
    }

    #[test]
    fn leyline_of_anticipation_lets_a_player_cast_a_sorcery_during_combat() -> TestResult {
        let (mut game, [p1, ..]) =
            MockGame::builder().turn(1, turns::Step::DeclareAttackers).players([
                P1.battlefield([LEYLINE_OF_ANTICIPATION]).hand([DIVINATION]).mana_pool(mana![2, U]),
                P2,
            ]);
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION)?;

        game.play([p1.cast(divination)])?;

        assert!(game.stack().has(divination));
        Ok(())
    }

    #[test]
    fn leyline_of_anticipation_lets_a_player_cast_a_sorcery_while_the_stack_is_not_empty()
    -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([LEYLINE_OF_ANTICIPATION])
                .hand([LIGHTNING_BOLT, DIVINATION])
                .mana_pool(mana![R, 2, U]),
            P2.hand([DISPEL]).mana_pool(mana![U]),
        ]);
        let [bolt] = game.player(p1).cards_in(HAND).exactly::<1>(LIGHTNING_BOLT)?;
        let [dispel] = game.player(p2).cards_in(HAND).exactly::<1>(DISPEL)?;
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION)?;

        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.cast(dispel).targeting(bolt),
        ])?;
        game.play([p2.pass_priority(), p1.cast(divination)])?;

        assert!(game.stack().has(divination));
        Ok(())
    }

    #[test]
    fn leyline_of_anticipation_does_not_let_an_opponent_cast_a_sorcery_at_instant_speed()
    -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([LEYLINE_OF_ANTICIPATION]),
            P2.hand([DIVINATION]).mana_pool(mana![2, U]),
        ]);
        let [divination] = game.player(p2).cards_in(HAND).exactly::<1>(DIVINATION)?;

        game.play([p1.pass_priority()])?;
        game.act(p2, actions::Action::Cast(divination.into()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotCast {
                spell: divination.into()
            })
        );
        assert!(game.player(p2).cards_in(HAND).has(divination));
        assert!(!game.stack().has(divination));
        Ok(())
    }

    #[test]
    fn a_land_cannot_be_cast_as_a_spell_even_with_leyline_of_anticipation() -> TestResult {
        let (mut game, [p1, ..]) =
            MockGame::new([P1.battlefield([LEYLINE_OF_ANTICIPATION]).hand([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;

        game.act(p1, actions::Action::Cast(forest.into()));

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::CannotCast {
                spell: forest.into()
            })
        );
        assert!(game.player(p1).cards_in(HAND).has(forest));
        assert!(!game.stack().has(forest));
        Ok(())
    }
}
