use verdigris_datalog_ir::Atom;
use verdigris_datalog_ir::Literal;
use verdigris_datalog_ir::Term;
use verdigris_datalog_ir::Value;
use verdigris_rule_engine::HAND;
use verdigris_rule_engine::cards;
use verdigris_rule_engine::events;
use verdigris_rule_engine::rules;

use crate::PlayDatalog;
use crate::TranslationError;

pub fn translate_condition(condition: &rules::Condition) -> Result<Vec<Literal>, TranslationError> {
    if *condition == rules::main_phase() {
        return Ok(vec![
            Literal::Pos(Atom {
                rel: PlayDatalog::STEP,
                terms: vec![Term::Var("S")],
            }),
            Literal::Neg(Atom {
                rel: PlayDatalog::MAIN_STEP,
                terms: vec![Term::Var("S")],
            }),
        ]);
    }
    if *condition == rules::stack_empty() {
        return Ok(vec![Literal::Pos(Atom {
            rel: PlayDatalog::ON_STACK,
            terms: vec![Term::Var("X")],
        })]);
    }
    let is_player =
        |reference: &rules::ArgRef| *reference == rules::ArgRef::Arg(events::Play::PLAYER);
    let negated = |relation: verdigris_datalog_ir::Rel, terms: Vec<Term>| {
        vec![Literal::Neg(Atom {
            rel: relation,
            terms,
        })]
    };
    match condition {
        rules::Condition::HasPriority(reference) if is_player(reference) => Ok(negated(
            PlayDatalog::PRIORITY,
            vec![Term::Var(PlayDatalog::PLAYER_VAR)],
        )),
        rules::Condition::ActivePlayer(reference) if is_player(reference) => Ok(negated(
            PlayDatalog::ACTIVE,
            vec![Term::Var(PlayDatalog::PLAYER_VAR)],
        )),
        rules::Condition::LandPlayAvailable(reference) if is_player(reference) => Ok(negated(
            PlayDatalog::LAND_PLAY_AVAILABLE,
            vec![Term::Var(PlayDatalog::PLAYER_VAR)],
        )),
        rules::Condition::Of(card, inner) if *card == rules::ArgRef::Arg(events::Play::CARD) => {
            match inner.as_ref() {
                rules::Condition::CardType(card_type) if *card_type == cards::CardType::Land => {
                    Ok(negated(
                        PlayDatalog::CARD_TYPE,
                        vec![
                            Term::Var(PlayDatalog::CARD_VAR),
                            Term::Val(Value::Sym(PlayDatalog::LAND)),
                        ],
                    ))
                }
                rules::Condition::InZone(zone, holder) if *zone == HAND && is_player(holder) => {
                    Ok(negated(
                        PlayDatalog::IN_ZONE,
                        vec![
                            Term::Var(PlayDatalog::CARD_VAR),
                            Term::Val(Value::Sym(PlayDatalog::HAND_ZONE)),
                            Term::Var(PlayDatalog::PLAYER_VAR),
                        ],
                    ))
                }
                rules::Condition::PointsTo(component, holder)
                    if *component == rules::ComponentRef::relation::<cards::Owner>()
                        && is_player(holder) =>
                {
                    Ok(negated(
                        PlayDatalog::OWNER,
                        vec![
                            Term::Var(PlayDatalog::CARD_VAR),
                            Term::Var(PlayDatalog::PLAYER_VAR),
                        ],
                    ))
                }
                _ => Err(TranslationError::Untranslatable(condition.clone())),
            }
        }
        _ => Err(TranslationError::Untranslatable(condition.clone())),
    }
}

#[cfg(test)]
mod tests {
    use verdigris_datalog_ir::Atom;
    use verdigris_datalog_ir::Literal;
    use verdigris_datalog_ir::Term;
    use verdigris_datalog_ir::Value;
    use verdigris_rule_engine::cards;
    use verdigris_rule_engine::events;
    use verdigris_rule_engine::rules;
    use verdigris_rule_engine::turns;

    use super::translate_condition;
    use crate::PlayDatalog;
    use crate::TranslationError;

    #[test]
    fn test_usage() {
        assert_eq!(
            translate_condition(&rules::has_priority(events::Play::PLAYER)),
            Ok(vec![Literal::Neg(Atom {
                rel: PlayDatalog::PRIORITY,
                terms: vec![Term::Var(PlayDatalog::PLAYER_VAR)],
            })])
        );
        assert_eq!(
            translate_condition(&rules::card_type(cards::CardType::Land).of(events::Play::CARD)),
            Ok(vec![Literal::Neg(Atom {
                rel: PlayDatalog::CARD_TYPE,
                terms: vec![
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Val(Value::Sym(PlayDatalog::LAND)),
                ],
            })])
        );
        assert_eq!(
            translate_condition(&rules::stack_empty()),
            Ok(vec![Literal::Pos(Atom {
                rel: PlayDatalog::ON_STACK,
                terms: vec![Term::Var("X")],
            })])
        );
        assert_eq!(
            translate_condition(&rules::main_phase()),
            Ok(vec![
                Literal::Pos(Atom {
                    rel: PlayDatalog::STEP,
                    terms: vec![Term::Var("S")],
                }),
                Literal::Neg(Atom {
                    rel: PlayDatalog::MAIN_STEP,
                    terms: vec![Term::Var("S")],
                }),
            ])
        );
    }

    #[test]
    fn an_untranslatable_atom_is_an_explicit_error() {
        let tapped = rules::has::<cards::Tapped>().of(events::Play::CARD);
        assert_eq!(
            translate_condition(&tapped),
            Err(TranslationError::Untranslatable(tapped.clone()))
        );
        assert_eq!(
            translate_condition(&rules::step_is(turns::Step::Upkeep)),
            Err(TranslationError::Untranslatable(rules::step_is(
                turns::Step::Upkeep
            )))
        );
    }
}
