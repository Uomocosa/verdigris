use bevy::prelude::Entity;

use crate::cards;
use crate::rules;
use crate::zones;
use frogmyr::Facts;
use frogmyr::Value;

use crate::legality;

pub fn entity_facts(answers: &rules::Rules, facts: &mut Facts, entity: Entity) {
    let zone_name = |zone: zones::Zone| match zone {
        zones::Zone::Library => "Library",
        zones::Zone::Hand => "Hand",
        zones::Zone::Battlefield => "Battlefield",
        zones::Zone::Graveyard => "Graveyard",
        zones::Zone::Exile => "Exile",
        zones::Zone::Stack => "Stack",
    };
    let _ = facts.add(
        legality::PlayDatalog::OBJECT,
        vec![Value::Entity(entity.to_bits())],
    );
    if let Ok(definition) = answers.definitions.get(entity) {
        for card_type in &definition.type_line.types {
            let _ = facts.add(
                legality::PlayDatalog::CARD_TYPE,
                vec![
                    Value::Entity(entity.to_bits()),
                    Value::Sym(legality::PlayDatalog::card_type_name(*card_type)),
                ],
            );
        }
    }
    let zone_lookup = answers
        .entities
        .get(entity)
        .ok()
        .and_then(|found| found.get::<zones::InZone>().copied())
        .and_then(|in_zone| answers.zones.get(*in_zone).ok());
    if let Some((_, zone, owner)) = zone_lookup {
        if *zone == zones::Zone::Stack {
            let _ = facts.add(
                legality::PlayDatalog::ON_STACK,
                vec![Value::Entity(entity.to_bits())],
            );
        }
        if let Some(owner) = owner {
            let _ = facts.add(
                legality::PlayDatalog::IN_ZONE,
                vec![
                    Value::Entity(entity.to_bits()),
                    Value::Sym(zone_name(*zone)),
                    Value::Entity((***owner).to_bits()),
                ],
            );
        }
    }
    if let Some(owner) =
        answers.entities.get(entity).ok().and_then(|found| found.get::<cards::Owner>())
    {
        let _ = facts.add(
            legality::PlayDatalog::OWNER,
            vec![
                Value::Entity(entity.to_bits()),
                Value::Entity((***owner).to_bits()),
            ],
        );
    }
    if let Ok(found) = answers.entities.get(entity) {
        if found.contains::<cards::OnBattlefield>() {
            for keyword in &answers.characteristics(entity).keywords {
                let _ = facts.add(
                    legality::PlayDatalog::HAS_KEYWORD,
                    vec![Value::Entity(entity.to_bits()), Value::Sym(*keyword.id)],
                );
            }
        }
        if let Some(effective) = found.get::<cards::EffectivePT>() {
            let _ = facts.add(
                legality::PlayDatalog::CHARACTERISTIC,
                vec![
                    Value::Entity(entity.to_bits()),
                    Value::Sym("Power"),
                    Value::Int(i64::from(effective.power)),
                ],
            );
            let _ = facts.add(
                legality::PlayDatalog::CHARACTERISTIC,
                vec![
                    Value::Entity(entity.to_bits()),
                    Value::Sym("Toughness"),
                    Value::Int(i64::from(effective.toughness)),
                ],
            );
        }
        let controller = rules::ComponentRef::relation::<cards::Controller>();
        if let Some(target) = controller.target.and_then(|read| read(&found)) {
            let _ = facts.add(
                legality::PlayDatalog::POINTS_TO,
                vec![
                    Value::Entity(entity.to_bits()),
                    Value::Sym(controller.name),
                    Value::Entity(target.to_bits()),
                ],
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::cards;
    use crate::keywords;
    use crate::legality;
    use crate::rules;
    use crate::testing::FOREST;
    use crate::testing::MockGame;
    use crate::testing::P1;
    use crate::testing::P2;
    use crate::testing::WIND_DRAKE;
    use frogmyr::Facts;
    use frogmyr::Term;
    use frogmyr::Value;

    use super::entity_facts;

    #[test]
    fn test_usage() {
        let (mut game, [p1, _p2]) =
            MockGame::new([P1.hand([FOREST]).battlefield([WIND_DRAKE]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        let [drake] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE).unwrap();
        game.pass_priority();
        let _ = game.read_rules(|answers, _legal| {
            let mut drake_facts = Facts::default();
            entity_facts(answers, &mut drake_facts, *drake);
            assert!(drake_facts.contains(
                &legality::PlayDatalog::OBJECT,
                &[Value::Entity(drake.to_bits())]
            ));
            assert!(drake_facts.contains(
                &legality::PlayDatalog::CARD_TYPE,
                &[Value::Entity(drake.to_bits()), Value::Sym("Creature")]
            ));
            assert!(drake_facts.contains(
                &legality::PlayDatalog::HAS_KEYWORD,
                &[
                    Value::Entity(drake.to_bits()),
                    Value::Sym(*keywords::flying().id)
                ]
            ));
            assert!(drake_facts.contains(
                &legality::PlayDatalog::CHARACTERISTIC,
                &[
                    Value::Entity(drake.to_bits()),
                    Value::Sym("Power"),
                    Value::Int(2)
                ]
            ));
            assert!(drake_facts.contains(
                &legality::PlayDatalog::CHARACTERISTIC,
                &[
                    Value::Entity(drake.to_bits()),
                    Value::Sym("Toughness"),
                    Value::Int(2)
                ]
            ));
            let controller = rules::ComponentRef::relation::<cards::Controller>();
            assert!(drake_facts.contains(
                &legality::PlayDatalog::POINTS_TO,
                &[
                    Value::Entity(drake.to_bits()),
                    Value::Sym(controller.name),
                    Value::Entity((*p1).to_bits()),
                ]
            ));
            let mut forest_facts = Facts::default();
            entity_facts(answers, &mut forest_facts, *forest);
            assert!(forest_facts.contains(
                &legality::PlayDatalog::OBJECT,
                &[Value::Entity(forest.to_bits())]
            ));
            assert_eq!(
                forest_facts.matching(
                    &legality::PlayDatalog::HAS_KEYWORD,
                    &[Term::Val(Value::Entity(forest.to_bits())), Term::Var("K")]
                ),
                Vec::<&Vec<Value>>::new()
            );
        });
    }
}
