use crate::cards;
use crate::events;
use crate::players;
use crate::rules;
use frogmyr::Facts;
use frogmyr::Value;

use crate::legality;

pub fn snapshot(answers: &rules::Rules, viewer: players::PlayerId) -> Facts {
    let mut facts = legality::PlayDatalog::snapshot(answers, viewer);
    let players_in_game = answers.players_in_game(rules::Players::All, Some(viewer));
    for card in legality::PlayDatalog::visible_entities(answers, viewer) {
        if let Ok(found) = answers.entities.get(card) {
            if found.get::<cards::Creature>().is_some() {
                let _ = facts.add(
                    legality::AttackDatalog::HAS,
                    vec![
                        Value::Entity(card.to_bits()),
                        Value::Sym(legality::AttackDatalog::CREATURE),
                    ],
                );
            }
            if found.get::<cards::OnBattlefield>().is_some() {
                let _ = facts.add(
                    legality::AttackDatalog::HAS,
                    vec![
                        Value::Entity(card.to_bits()),
                        Value::Sym(legality::AttackDatalog::ON_BATTLEFIELD),
                    ],
                );
            }
            if found.get::<cards::Tapped>().is_some() {
                let _ = facts.add(
                    legality::AttackDatalog::HAS,
                    vec![
                        Value::Entity(card.to_bits()),
                        Value::Sym(legality::AttackDatalog::TAPPED),
                    ],
                );
            }
            if found.get::<cards::SummoningSick>().is_some() {
                let _ = facts.add(
                    legality::AttackDatalog::HAS,
                    vec![
                        Value::Entity(card.to_bits()),
                        Value::Sym(legality::AttackDatalog::SUMMONING_SICK),
                    ],
                );
            }
        }
        for player in &players_in_game {
            let event = events::Attack { attacker: card };
            legality::PlayDatalog::pair_facts_for(
                answers,
                &mut facts,
                &event,
                card,
                **player,
                legality::AttackDatalog::ATTACK,
                &rules::ArgRef::Arg(events::Attack::ATTACKER),
                &rules::ArgRef::Arg(events::Attack::ATTACKER),
            );
        }
    }
    facts
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::legality;
    use crate::players;
    use crate::testing::GRIZZLY_BEARS;
    use crate::testing::MockGame;
    use crate::testing::P1;
    use crate::testing::P2;
    use frogmyr::Value;

    use super::snapshot;

    #[test]
    fn test_usage() {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.pass_priority();
        game.read_rules(|answers, _legal| {
            let facts = snapshot(answers, players::PlayerId(*p1));
            assert!(facts.contains(
                &legality::PlayDatalog::OBJECT,
                &[Value::Entity(bears.to_bits())]
            ));
            assert!(facts.contains(
                &legality::AttackDatalog::HAS,
                &[Value::Entity(bears.to_bits()), Value::Sym("creature")]
            ));
            assert!(facts.contains(
                &legality::AttackDatalog::HAS,
                &[Value::Entity(bears.to_bits()), Value::Sym("on_battlefield")]
            ));
            assert!(!facts.contains(
                &legality::AttackDatalog::HAS,
                &[Value::Entity(bears.to_bits()), Value::Sym("tapped")]
            ));
            assert!(!facts.contains(
                &legality::AttackDatalog::HAS,
                &[Value::Entity(bears.to_bits()), Value::Sym("summoning_sick")]
            ));
        })
        .unwrap();
    }
}
