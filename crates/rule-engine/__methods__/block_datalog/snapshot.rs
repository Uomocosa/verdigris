use crate::cards;
use crate::events;
use crate::players;
use crate::rules;
use frogmyr::Facts;
use frogmyr::Value;

use crate::legality;

pub fn snapshot(answers: &rules::Rules, viewer: players::PlayerId) -> Facts {
    let mut facts = legality::PlayDatalog::snapshot(answers, viewer);
    let visible = legality::PlayDatalog::visible_entities(answers, viewer);
    for card in &visible {
        if let Ok(found) = answers.entities.get(*card) {
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
        }
    }
    for blocker in &visible {
        for attacker in &visible {
            let event = events::DeclareBlocker {
                blocker: *blocker,
                attacker: *attacker,
            };
            legality::PlayDatalog::pair_facts_for(
                answers,
                &mut facts,
                &event,
                *blocker,
                *attacker,
                legality::BlockDatalog::BLOCK,
                &rules::ArgRef::Arg(events::DeclareBlocker::BLOCKER),
                &rules::ArgRef::Arg(events::DeclareBlocker::ATTACKER),
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
        })
        .unwrap();
    }
}
