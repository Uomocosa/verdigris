use std::collections::BTreeSet;

use bevy::prelude::{Entity, MessageReader, MessageWriter, Query, Res, ResMut};

use crate::BATTLEFIELD;
use crate::HAND;
use crate::actions;
use crate::cards;
use crate::players;
use crate::rules;
use crate::turns;
use crate::zones;

/// 103.6. Some cards allow a player to take actions with them from their opening hand. Once the mulligan process
/// (see rule 103.5) is complete, the starting player may take any such actions in any order. Then each other
/// player in turn order may do the same.
///
/// 103.6a If a card allows a player to begin the game with that card on the battlefield, the player taking this
/// action puts that card onto the battlefield.
pub fn r103_6_r103_6a_opening_hand_actions(
    mut actions: MessageReader<actions::PlayerAction>,
    mut rejected: MessageWriter<actions::Rejected>,
    mut pending: ResMut<actions::PendingChoice>,
    mut window: ResMut<actions::OpeningHandWindow>,
    mut entering: MessageWriter<rules::Would<rules::EnterBattlefield>>,
    turn: Res<turns::Turn>,
    turn_order: Res<turns::TurnOrder>,
    opening_hands: Query<(
        Entity,
        &cards::CardDefinition,
        &cards::Owner,
        &zones::InZone,
    )>,
    zones: zones::ZoneLookup,
) {
    let turn = turn.into_inner();
    let turn_order = turn_order.into_inner();
    if pending.is_none() {
        let player_count = turn_order.len();
        if window.len() < player_count {
            let mut current = turn.active;
            for _ in 0..player_count {
                if !window.contains(&current) {
                    window.push(current);
                    let eligible = opening_hand_cards(current, &opening_hands, &zones);
                    if !eligible.is_empty() {
                        *pending =
                            actions::PendingChoice(Some(actions::Choice::OpeningHandActions(
                                actions::OpeningHandActionsChoice {
                                    player: current,
                                    cards: eligible,
                                },
                            )));
                        break;
                    }
                }
                let Some(next) = turn_order.next_after(current) else {
                    break;
                };
                current = next;
            }
        }
    }
    for action in actions.read() {
        let actions::Action::BeginGameWithOnBattlefield(answered) = &action.action else {
            continue;
        };
        let Some(actions::Choice::OpeningHandActions(choice)) = (*pending).as_ref().cloned() else {
            rejected.write(actions::Rejected::InvalidOpeningHandActions {
                player: action.player,
            });
            continue;
        };
        let legal: BTreeSet<Entity> = choice.cards.iter().copied().collect();
        let received: BTreeSet<Entity> = answered.iter().copied().collect();
        let is_chooser = action.player == choice.player;
        if !(is_chooser && received.is_subset(&legal)) {
            rejected.write(actions::Rejected::InvalidOpeningHandActions {
                player: action.player,
            });
            continue;
        }
        if zones.find(BATTLEFIELD, Some(action.player)).is_none() {
            rejected.write(actions::Rejected::InvalidOpeningHandActions {
                player: action.player,
            });
            continue;
        }
        for card in answered {
            entering.write(rules::Would(rules::EnterBattlefield {
                permanent: *card,
                controller: action.player,
                summoning_sick: false,
            }));
        }
        *pending = actions::PendingChoice(None);
    }
}

fn opening_hand_cards(
    player: players::PlayerId,
    opening_hands: &Query<(
        Entity,
        &cards::CardDefinition,
        &cards::Owner,
        &zones::InZone,
    )>,
    zones: &zones::ZoneLookup,
) -> Vec<Entity> {
    let Some(hand) = zones.find(HAND, Some(player)) else {
        return Vec::new();
    };
    opening_hands
        .iter()
        .filter(|(_, definition, owner, in_zone)| {
            ***owner == player
                && ***in_zone == hand
                && definition.begins_the_game_on_the_battlefield()
        })
        .map(|(entity, _, _, _)| entity)
        .collect()
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
    fn casting_while_the_opening_hand_window_is_open_is_rejected() -> TestResult {
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
    fn playing_a_land_while_the_opening_hand_window_is_open_is_rejected() -> TestResult {
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
    fn activating_a_mana_ability_while_the_opening_hand_window_is_open_is_rejected() -> TestResult {
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
    fn passing_priority_while_the_opening_hand_window_is_open_is_rejected() {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([LEYLINE_OF_ANTICIPATION]), P2]);

        game.act(p1, actions::Action::PassPriority);

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::NoPriority { player: p1.id })
        );
        assert_eq!(game.priority_player(), Some(p1.id));
    }

    #[test]
    fn declaring_attackers_while_the_opening_hand_window_is_open_is_rejected() -> TestResult {
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
    fn leyline_in_the_opening_hand_begins_the_game_on_the_battlefield() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([
            P1.hand([LEYLINE_OF_ANTICIPATION, DIVINATION]).battlefield([TUNDRA, TUNDRA, TUNDRA]),
            P2,
        ]);
        let [leyline] = game.player(p1).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;

        game.act(
            p1,
            actions::Action::BeginGameWithOnBattlefield(vec![leyline.into()]),
        );

        assert!(game.player(p1).cards_in(BATTLEFIELD).has(leyline));
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
        game.act(
            p1,
            actions::Action::BeginGameWithOnBattlefield(vec![leyline.into()]),
        );
        assert!(game.player(p1).cards_in(BATTLEFIELD).has(leyline));
        let [tower_1, tower_2] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(RELIQUARY_TOWER)?;
        let [tundra] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(TUNDRA)?;
        let [divination] = game.player(p1).cards_in(HAND).exactly::<1>(DIVINATION)?;

        game.play([
            p2.pass_priority(),
            p1.activate_ability(tower_1.mana_ability(cards::ManaType::Colorless)),
            p1.activate_ability(tower_2.mana_ability(cards::ManaType::Colorless)),
            p1.activate_ability(tundra.mana_ability(cards::ManaType::from(cards::Color::Blue))),
            p1.cast(divination),
        ])?;

        assert!(game.stack().has(divination));
        assert_eq!(game.mana_pool(p1), cards::ManaPool::EMPTY);
        Ok(())
    }

    #[test]
    fn passing_the_opening_hand_choice_leaves_leyline_in_hand() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([P1.hand([LEYLINE_OF_ANTICIPATION]), P2]);
        let [leyline] = game.player(p1).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;

        game.act(p1, actions::Action::BeginGameWithOnBattlefield(Vec::new()));

        assert!(game.player(p1).cards_in(HAND).has(leyline));
        assert!(game.player(p1).cards_in(BATTLEFIELD).has_no(LEYLINE_OF_ANTICIPATION));
        assert!(game.last_rejection().is_none());
        game.play([p1.pass_priority(), p2.pass_priority()])?;
        assert_eq!(game.turn().step, turns::Step::BeginningOfCombat);
        Ok(())
    }

    #[test]
    fn opening_hand_choices_follow_turn_order_with_three_players() -> TestResult {
        let (mut game, [p1, p2, p3]) = MockGame::new([
            P1.hand([LEYLINE_OF_ANTICIPATION]),
            P2.hand([LEYLINE_OF_ANTICIPATION]),
            P3.hand([LEYLINE_OF_ANTICIPATION]),
        ]);
        let [p1_leyline] = game.player(p1).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;
        let [p2_leyline] = game.player(p2).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;
        let [p3_leyline] = game.player(p3).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;

        game.act(
            p3,
            actions::Action::BeginGameWithOnBattlefield(vec![p3_leyline.into()]),
        );

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::InvalidOpeningHandActions { player: p3.id })
        );
        assert!(game.player(p3).cards_in(HAND).has(p3_leyline));

        game.act(
            p1,
            actions::Action::BeginGameWithOnBattlefield(vec![p1_leyline.into()]),
        );
        game.act(
            p2,
            actions::Action::BeginGameWithOnBattlefield(vec![p2_leyline.into()]),
        );
        game.act(
            p3,
            actions::Action::BeginGameWithOnBattlefield(vec![p3_leyline.into()]),
        );

        assert!(game.player(p1).cards_in(BATTLEFIELD).has(p1_leyline));
        assert!(game.player(p2).cards_in(BATTLEFIELD).has(p2_leyline));
        assert!(game.player(p3).cards_in(BATTLEFIELD).has(p3_leyline));
        let window = game.app.world().resource::<actions::OpeningHandWindow>();
        assert_eq!(window.len(), 3);
        Ok(())
    }

    #[test]
    fn opening_hand_choices_follow_turn_order() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LEYLINE_OF_ANTICIPATION]),
            P2.hand([LEYLINE_OF_ANTICIPATION]),
        ]);
        let [p1_leyline] = game.player(p1).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;
        let [p2_leyline] = game.player(p2).cards_in(HAND).exactly::<1>(LEYLINE_OF_ANTICIPATION)?;

        game.act(
            p2,
            actions::Action::BeginGameWithOnBattlefield(vec![p2_leyline.into()]),
        );

        assert_eq!(
            game.last_rejection(),
            Some(&actions::Rejected::InvalidOpeningHandActions { player: p2.id })
        );
        assert!(game.player(p2).cards_in(HAND).has(p2_leyline));

        game.act(
            p1,
            actions::Action::BeginGameWithOnBattlefield(vec![p1_leyline.into()]),
        );

        assert!(game.player(p1).cards_in(BATTLEFIELD).has(p1_leyline));
        let window = game.app.world().resource::<actions::OpeningHandWindow>();
        assert_eq!(window.len(), 1);
        assert_eq!(window[0], p1.id);

        game.act(
            p2,
            actions::Action::BeginGameWithOnBattlefield(vec![p2_leyline.into()]),
        );

        assert!(game.player(p2).cards_in(BATTLEFIELD).has(p2_leyline));
        let window = game.app.world().resource::<actions::OpeningHandWindow>();
        assert_eq!(window.len(), 2);
        Ok(())
    }
}
