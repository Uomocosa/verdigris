use bevy::prelude::{Res, ResMut};

use crate::Game;
use crate::ORDER_TRIGGERS;
use crate::actions;
use crate::events;
use crate::rules;

pub fn refresh_legal_actions(
    mut legal: ResMut<actions::LegalActions>,
    pending: Res<actions::PendingChoice>,
    game: Game,
    answers: rules::Rules,
) {
    let pending = pending.into_inner();
    let actions::PendingChoice(choice) = pending;
    if let Some(choice) = choice {
        match choice {
            actions::Choice::Pick(pick) if pick.id == ORDER_TRIGGERS => {
                *legal = actions::LegalActions {
                    player: Some(pick.player),
                    actions: Vec::new(),
                };
            }
            actions::Choice::Pick(pick) => {
                let previews: Vec<(actions::Action, rules::Legality)> = pick
                    .among
                    .iter()
                    .map(|entity| {
                        (
                            actions::Action::Choose(vec![*entity]),
                            rules::Legality::Legal,
                        )
                    })
                    .collect();
                *legal = actions::LegalActions {
                    player: Some(pick.player),
                    actions: previews,
                };
            }
            actions::Choice::CombatDamage(_) => {
                *legal = actions::LegalActions {
                    player: Some(choice.player()),
                    actions: Vec::new(),
                };
            }
        }
        return;
    }
    let Some(holder) = game.priority_holder() else {
        *legal = actions::LegalActions {
            player: None,
            actions: Vec::new(),
        };
        return;
    };
    let player = holder.id;
    let mut refreshed: Vec<(actions::Action, rules::Legality)> = Vec::new();
    refreshed.push((actions::Action::PassPriority, rules::Legality::Legal));
    refreshed.push((actions::Action::Concede, rules::Legality::Legal));
    for (event, legality) in answers.solve::<events::Play>(
        &[(events::Play::PLAYER, rules::ArgValue::Player(player))],
        player,
    ) {
        refreshed.push((actions::Action::Play(event.card), legality));
    }
    for (event, legality) in answers.solve::<events::Cast>(
        &[(events::Cast::CASTER, rules::ArgValue::Player(player))],
        player,
    ) {
        refreshed.push((actions::Action::Cast(event.spell), legality));
    }
    *legal = actions::LegalActions {
        player: Some(player),
        actions: refreshed,
    };
}
