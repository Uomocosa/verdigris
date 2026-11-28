use frogmyr::Facts;
use frogmyr::Value;
use verdigris_rule_engine::cards;
use verdigris_rule_engine::events;
use verdigris_rule_engine::players;
use verdigris_rule_engine::rules;

use crate::AttackDatalog;
use crate::BlockDatalog;
use crate::PlayDatalog;

pub fn snapshot(answers: &rules::Rules, viewer: players::PlayerId) -> Facts {
    let mut facts = PlayDatalog::snapshot(answers, viewer);
    let visible = PlayDatalog::visible_entities(answers, viewer);
    for card in &visible {
        if let Ok(found) = answers.entities.get(*card) {
            if found.get::<cards::Creature>().is_some() {
                let _ = facts.add(
                    AttackDatalog::HAS,
                    vec![
                        Value::Entity(card.to_bits()),
                        Value::Sym(AttackDatalog::CREATURE),
                    ],
                );
            }
            if found.get::<cards::OnBattlefield>().is_some() {
                let _ = facts.add(
                    AttackDatalog::HAS,
                    vec![
                        Value::Entity(card.to_bits()),
                        Value::Sym(AttackDatalog::ON_BATTLEFIELD),
                    ],
                );
            }
            if found.get::<cards::Tapped>().is_some() {
                let _ = facts.add(
                    AttackDatalog::HAS,
                    vec![
                        Value::Entity(card.to_bits()),
                        Value::Sym(AttackDatalog::TAPPED),
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
            PlayDatalog::pair_facts_for(
                answers,
                &mut facts,
                &event,
                *blocker,
                *attacker,
                BlockDatalog::BLOCK,
            );
        }
    }
    facts
}

#[cfg(test)]
mod tests {
    use frogmyr::Value;
    use verdigris_rule_engine::BATTLEFIELD;
    use verdigris_rule_engine::players;
    use verdigris_rule_engine::testing::GRIZZLY_BEARS;
    use verdigris_rule_engine::testing::MockGame;
    use verdigris_rule_engine::testing::P1;
    use verdigris_rule_engine::testing::P2;

    use super::snapshot;
    use crate::AttackDatalog;
    use crate::PlayDatalog;

    #[test]
    fn test_usage() {
        let (mut game, [p1, _]) = MockGame::new([P1.battlefield([GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.pass_priority();
        game.read_rules(|answers, _legal| {
            let facts = snapshot(answers, players::PlayerId(*p1));
            assert!(facts.contains(&PlayDatalog::OBJECT, &[Value::Entity(bears.to_bits())]));
            assert!(facts.contains(
                &AttackDatalog::HAS,
                &[Value::Entity(bears.to_bits()), Value::Sym("creature")]
            ));
            assert!(facts.contains(
                &AttackDatalog::HAS,
                &[Value::Entity(bears.to_bits()), Value::Sym("on_battlefield")]
            ));
            assert!(!facts.contains(
                &AttackDatalog::HAS,
                &[Value::Entity(bears.to_bits()), Value::Sym("tapped")]
            ));
        })
        .unwrap();
    }
}
