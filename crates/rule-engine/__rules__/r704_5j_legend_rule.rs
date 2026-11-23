use std::collections::BTreeMap;

use bevy::prelude::{Entity, MessageReader, MessageWriter, Query, ResMut, With};

use crate::StateBasedActionsPerformed;
use crate::actions;
use crate::cards;
use crate::rules;

/// 704.5j If two or more legendary permanents with the same name are controlled by the same
/// player, that player chooses one of them, and the rest are put into their owners’ graveyards.
/// This is called the “legend rule.”
pub fn r704_5j_legend_rule(
    mut destructions: MessageWriter<rules::Would<rules::Destroy>>,
    mut pending: ResMut<actions::PendingChoice>,
    mut performed: ResMut<StateBasedActionsPerformed>,
    mut actions: MessageReader<actions::PlayerAction>,
    permanents: Query<(Entity, &cards::CardDefinition, &cards::Owner), With<cards::OnBattlefield>>,
) {
    for action in actions.read() {
        let actions::Action::ChooseLegendary(chosen) = &action.action else {
            continue;
        };
        let Some(actions::Choice::Legendary(choice)) = (*pending).as_ref().cloned() else {
            continue;
        };
        if action.player != choice.player || !choice.legal.contains(chosen) {
            continue;
        }
        *pending = actions::PendingChoice(None);
        for other in choice.legal.iter().copied().filter(|other| other != chosen) {
            destructions.write(rules::Would(rules::Destroy {
                permanent: other,
                controller: choice.player,
            }));
        }
        **performed = true;
        return;
    }

    let mut legendary_by_owner_and_name: BTreeMap<(Entity, &str), Vec<Entity>> = BTreeMap::new();
    for (entity, definition, owner) in permanents.iter() {
        if definition.type_line.supertypes.contains(&cards::Supertype::Legendary) {
            legendary_by_owner_and_name
                .entry((**owner, definition.name.as_str()))
                .or_default()
                .push(entity);
        }
    }
    for ((owner, _), entities) in legendary_by_owner_and_name {
        if entities.len() < 2 {
            continue;
        }
        if matches!(
            (*pending).as_ref(),
            Some(actions::Choice::Legendary(choice)) if choice.player == owner
        ) {
            return;
        }
        let legal = entities.into_iter().collect();
        *pending =
            actions::PendingChoice(Some(actions::Choice::Legendary(actions::LegendaryChoice {
                player: owner,
                legal,
            })));
        return;
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::GRAVEYARD;
    use crate::actions;
    use crate::testing::*;

    #[test]
    fn legend_rule_keeps_the_chosen_permanent_and_buries_the_other() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([
            P1.battlefield([ISAMARU_HOUND_OF_KONDA, ISAMARU_HOUND_OF_KONDA]),
            P2,
        ]);
        let [isamaru_1, isamaru_2] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(ISAMARU_HOUND_OF_KONDA)?;

        game.play([p1.pass_priority()])?;
        game.act(p1, actions::Action::ChooseLegendary(isamaru_1.into()));

        assert!(game.player(p1).cards_in(BATTLEFIELD).has(isamaru_1));
        assert!(game.player(p1).cards_in(GRAVEYARD).has(isamaru_2));
        Ok(())
    }
}
