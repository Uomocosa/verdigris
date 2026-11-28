use crate::cards;
use crate::rules;
use frogmyr::Atom;
use frogmyr::CmpOp;
use frogmyr::Literal;
use frogmyr::Term;
use frogmyr::Value;

use crate::legality;

pub fn translate_subject_condition(
    condition: &rules::Condition,
    subject: &'static str,
) -> Result<Option<Vec<Literal>>, legality::TranslationError> {
    let positive = |relation: frogmyr::Rel, terms: Vec<Term>| {
        vec![Literal::Pos(Atom {
            rel: relation,
            terms,
        })]
    };
    match condition {
        rules::Condition::HasKeyword(id) => Ok(Some(positive(
            legality::PlayDatalog::HAS_KEYWORD,
            vec![Term::Var(subject), Term::Val(Value::Sym(**id))],
        ))),
        rules::Condition::Characteristic(characteristic, comparison) => {
            let name = match characteristic {
                rules::Characteristic::Power => "Power",
                rules::Characteristic::Toughness => "Toughness",
            };
            let (operator, threshold) = match *comparison {
                rules::Comparison::OrMore(threshold) => (CmpOp::Ge, threshold),
                rules::Comparison::OrLess(threshold) => (CmpOp::Le, threshold),
                rules::Comparison::Exactly(threshold) => (CmpOp::Eq, threshold),
                rules::Comparison::GreaterThan(threshold) => (CmpOp::Gt, threshold),
                rules::Comparison::LessThan(threshold) => (CmpOp::Lt, threshold),
            };
            let mut literals = positive(
                legality::PlayDatalog::CHARACTERISTIC,
                vec![
                    Term::Var(subject),
                    Term::Val(Value::Sym(name)),
                    Term::Var("N"),
                ],
            );
            let threshold = i64::from(threshold);
            literals.push(Literal::Cmp(
                Term::Var("N"),
                operator,
                Term::Val(Value::Int(threshold)),
            ));
            Ok(Some(literals))
        }
        rules::Condition::CardType(card_type) => Ok(Some(positive(
            legality::PlayDatalog::CARD_TYPE,
            vec![
                Term::Var(subject),
                Term::Val(Value::Sym(legality::PlayDatalog::card_type_name(
                    *card_type,
                ))),
            ],
        ))),
        rules::Condition::Has(component) => {
            let component_name = if *component == rules::ComponentRef::of::<cards::Creature>() {
                legality::AttackDatalog::CREATURE
            } else if *component == rules::ComponentRef::of::<cards::OnBattlefield>() {
                legality::AttackDatalog::ON_BATTLEFIELD
            } else if *component == rules::ComponentRef::of::<cards::Tapped>() {
                legality::AttackDatalog::TAPPED
            } else if *component == rules::ComponentRef::of::<cards::SummoningSick>() {
                legality::AttackDatalog::SUMMONING_SICK
            } else {
                return Err(legality::TranslationError::Untranslatable(
                    condition.clone(),
                ));
            };
            Ok(Some(positive(
                legality::AttackDatalog::HAS,
                vec![Term::Var(subject), Term::Val(Value::Sym(component_name))],
            )))
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::keywords;
    use crate::legality;
    use crate::rules;
    use frogmyr::Atom;
    use frogmyr::CmpOp;
    use frogmyr::Literal;
    use frogmyr::Term;
    use frogmyr::Value;

    use super::translate_subject_condition;

    #[test]
    fn test_usage() {
        let flying = rules::has_keyword(keywords::flying().id);
        let flying_atom = |subject: &'static str| {
            vec![Literal::Pos(Atom {
                rel: legality::PlayDatalog::HAS_KEYWORD,
                terms: vec![
                    Term::Var(subject),
                    Term::Val(Value::Sym(*keywords::flying().id)),
                ],
            })]
        };
        assert_eq!(
            translate_subject_condition(&flying, legality::PlayDatalog::CARD_VAR),
            Ok(Some(flying_atom(legality::PlayDatalog::CARD_VAR)))
        );
        assert_eq!(
            translate_subject_condition(&flying, legality::PlayDatalog::PLAYER_VAR),
            Ok(Some(flying_atom(legality::PlayDatalog::PLAYER_VAR)))
        );
        let power_check = rules::power().is(rules::Value(3).or_more());
        assert_eq!(
            translate_subject_condition(&power_check, legality::PlayDatalog::CARD_VAR),
            Ok(Some(vec![
                Literal::Pos(Atom {
                    rel: legality::PlayDatalog::CHARACTERISTIC,
                    terms: vec![
                        Term::Var(legality::PlayDatalog::CARD_VAR),
                        Term::Val(Value::Sym("Power")),
                        Term::Var("N"),
                    ],
                }),
                Literal::Cmp(Term::Var("N"), CmpOp::Ge, Term::Val(Value::Int(3))),
            ]))
        );
        assert_eq!(
            translate_subject_condition(
                &rules::card_type(cards::CardType::Land),
                legality::PlayDatalog::CARD_VAR
            ),
            Ok(Some(vec![Literal::Pos(Atom {
                rel: legality::PlayDatalog::CARD_TYPE,
                terms: vec![
                    Term::Var(legality::PlayDatalog::CARD_VAR),
                    Term::Val(Value::Sym(legality::PlayDatalog::LAND)),
                ],
            })]))
        );
        assert_eq!(
            translate_subject_condition(
                &rules::has::<cards::Creature>(),
                legality::PlayDatalog::CARD_VAR
            ),
            Ok(Some(vec![Literal::Pos(Atom {
                rel: legality::AttackDatalog::HAS,
                terms: vec![
                    Term::Var(legality::PlayDatalog::CARD_VAR),
                    Term::Val(Value::Sym(legality::AttackDatalog::CREATURE)),
                ],
            })]))
        );
        assert_eq!(
            translate_subject_condition(
                &rules::has_priority(rules::ArgId("other")),
                legality::PlayDatalog::CARD_VAR
            ),
            Ok(None)
        );
        assert_eq!(
            translate_subject_condition(&rules::Condition::True, legality::PlayDatalog::CARD_VAR),
            Ok(None)
        );
    }
}
