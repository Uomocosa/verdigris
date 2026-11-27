use bevy::prelude::{Entity, Res, ResMut};

use crate::Game;
use crate::ORDER_TRIGGERS;
use crate::actions;
use crate::cards;
use crate::events;
use crate::rules;
use crate::turns;

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
                    attackers: Vec::new(),
                    blocks: Vec::new(),
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
                    attackers: Vec::new(),
                    blocks: Vec::new(),
                };
            }
            actions::Choice::CombatDamage(_) => {
                *legal = actions::LegalActions {
                    player: Some(choice.player()),
                    actions: Vec::new(),
                    attackers: Vec::new(),
                    blocks: Vec::new(),
                };
            }
        }
        return;
    }
    let Some(holder) = game.priority_holder() else {
        *legal = actions::LegalActions {
            player: None,
            actions: Vec::new(),
            attackers: Vec::new(),
            blocks: Vec::new(),
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
    let mut attackers: Vec<(Entity, rules::Legality)> = Vec::new();
    if game.active_player().is_some_and(|active_player| active_player.id == player)
        && game.turn().step() == Some(turns::Step::DeclareAttackers)
    {
        for candidate in answers.entities.iter() {
            if candidate.get::<cards::Creature>().is_some()
                && candidate
                    .get::<cards::Controller>()
                    .is_some_and(|controller| **controller == player)
            {
                let creature = candidate.id();
                let legality = match answers.validate_attackers(&game, player, &[creature]) {
                    Ok(()) => rules::Legality::Legal,
                    Err(whys) => rules::Legality::Illegal(
                        whys.into_iter()
                            .filter(|why| why.creature == Some(creature))
                            .map(|why| why.why)
                            .collect(),
                    ),
                };
                attackers.push((creature, legality));
            }
        }
    }
    let blocks = answers.legal_block_previews(&game, player);
    *legal = actions::LegalActions {
        player: Some(player),
        actions: refreshed,
        attackers,
        blocks,
    };
}
