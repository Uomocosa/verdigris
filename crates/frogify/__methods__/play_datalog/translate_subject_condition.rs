use frogmyr::Atom;
use frogmyr::CmpOp;
use frogmyr::Literal;
use frogmyr::Term;
use frogmyr::Value;
use verdigris_rule_engine::cards;
use verdigris_rule_engine::rules;

use crate::AttackDatalog;
use crate::PlayDatalog;
use crate::TranslationError;

pub fn translate_subject_condition(
    condition: &rules::Condition,
    subject: &'static str,
) -> Result<Option<Vec<Literal>>, TranslationError> {
    let positive = |relation: frogmyr::Rel, terms: Vec<Term>| {
        vec![Literal::Pos(Atom {
            rel: relation,
            terms,
        })]
    };
    match condition {
        rules::Condition::HasKeyword(id) => Ok(Some(positive(
            PlayDatalog::HAS_KEYWORD,
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
                PlayDatalog::CHARACTERISTIC,
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
            PlayDatalog::CARD_TYPE,
            vec![
                Term::Var(subject),
                Term::Val(Value::Sym(PlayDatalog::card_type_name(*card_type))),
            ],
        ))),
        rules::Condition::Has(component) => {
            let component_name = if *component == rules::ComponentRef::of::<cards::Creature>() {
                AttackDatalog::CREATURE
            } else if *component == rules::ComponentRef::of::<cards::OnBattlefield>() {
                AttackDatalog::ON_BATTLEFIELD
            } else if *component == rules::ComponentRef::of::<cards::Tapped>() {
                AttackDatalog::TAPPED
            } else if *component == rules::ComponentRef::of::<cards::SummoningSick>() {
                AttackDatalog::SUMMONING_SICK
            } else {
                return Err(TranslationError::Untranslatable(condition.clone()));
            };
            Ok(Some(positive(
                AttackDatalog::HAS,
                vec![Term::Var(subject), Term::Val(Value::Sym(component_name))],
            )))
        }
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use frogmyr::Atom;
    use frogmyr::CmpOp;
    use frogmyr::Literal;
    use frogmyr::Term;
    use frogmyr::Value;
    use verdigris_rule_engine::cards;
    use verdigris_rule_engine::keywords;
    use verdigris_rule_engine::rules;

    use super::translate_subject_condition;
    use crate::AttackDatalog;
    use crate::PlayDatalog;

    #[test]
    fn test_usage() {
        let flying = rules::has_keyword(keywords::flying().id);
        let flying_atom = |subject: &'static str| {
            vec![Literal::Pos(Atom {
                rel: PlayDatalog::HAS_KEYWORD,
                terms: vec![
                    Term::Var(subject),
                    Term::Val(Value::Sym(*keywords::flying().id)),
                ],
            })]
        };
        assert_eq!(
            translate_subject_condition(&flying, PlayDatalog::CARD_VAR),
            Ok(Some(flying_atom(PlayDatalog::CARD_VAR)))
        );
        assert_eq!(
            translate_subject_condition(&flying, PlayDatalog::PLAYER_VAR),
            Ok(Some(flying_atom(PlayDatalog::PLAYER_VAR)))
        );
        let power_check = rules::power().is(rules::Value(3).or_more());
        assert_eq!(
            translate_subject_condition(&power_check, PlayDatalog::CARD_VAR),
            Ok(Some(vec![
                Literal::Pos(Atom {
                    rel: PlayDatalog::CHARACTERISTIC,
                    terms: vec![
                        Term::Var(PlayDatalog::CARD_VAR),
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
                PlayDatalog::CARD_VAR
            ),
            Ok(Some(vec![Literal::Pos(Atom {
                rel: PlayDatalog::CARD_TYPE,
                terms: vec![
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Val(Value::Sym(PlayDatalog::LAND)),
                ],
            })]))
        );
        assert_eq!(
            translate_subject_condition(&rules::has::<cards::Creature>(), PlayDatalog::CARD_VAR),
            Ok(Some(vec![Literal::Pos(Atom {
                rel: AttackDatalog::HAS,
                terms: vec![
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Val(Value::Sym(AttackDatalog::CREATURE)),
                ],
            })]))
        );
        assert_eq!(
            translate_subject_condition(
                &rules::has_priority(rules::ArgId("other")),
                PlayDatalog::CARD_VAR
            ),
            Ok(None)
        );
        assert_eq!(
            translate_subject_condition(&rules::Condition::True, PlayDatalog::CARD_VAR),
            Ok(None)
        );
    }
}
