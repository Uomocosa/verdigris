use crate::rules;
use frogmyr::Atom;
use frogmyr::Literal;
use frogmyr::Rule;

use crate::legality;

pub fn translate_holds(
    condition: &rules::Condition,
    card: &rules::ArgRef,
    holder: &rules::ArgRef,
    subject: &'static str,
    candidate: &Atom,
    holds: &mut Vec<Rule>,
    next_hold: &mut usize,
) -> Result<Atom, legality::TranslationError> {
    fn recursion(
        condition: &rules::Condition,
        card: &rules::ArgRef,
        holder: &rules::ArgRef,
        subject: &'static str,
        candidate: &Atom,
        holds: &mut Vec<Rule>,
        next_hold: &mut usize,
    ) -> Result<Atom, legality::TranslationError> {
        match condition {
            rules::Condition::All(children) => {
                let head = legality::PlayDatalog::holds_head(*next_hold)?;
                *next_hold = next_hold.saturating_add(1);
                let mut body = vec![Literal::Pos(candidate.clone())];
                for child in children {
                    body.push(Literal::Pos(recursion(
                        child, card, holder, subject, candidate, holds, next_hold,
                    )?));
                }
                holds.push(Rule {
                    head: head.clone(),
                    body,
                });
                Ok(head)
            }
            rules::Condition::Any(children) if *condition != rules::main_phase() => {
                let head = legality::PlayDatalog::holds_head(*next_hold)?;
                *next_hold = next_hold.saturating_add(1);
                for child in children {
                    let child_head =
                        recursion(child, card, holder, subject, candidate, holds, next_hold)?;
                    holds.push(Rule {
                        head: head.clone(),
                        body: vec![Literal::Pos(candidate.clone()), Literal::Pos(child_head)],
                    });
                }
                Ok(head)
            }
            rules::Condition::Not(inner) => {
                let head = legality::PlayDatalog::holds_head(*next_hold)?;
                *next_hold = next_hold.saturating_add(1);
                let child_head =
                    recursion(inner, card, holder, subject, candidate, holds, next_hold)?;
                holds.push(Rule {
                    head: head.clone(),
                    body: vec![Literal::Pos(candidate.clone()), Literal::Neg(child_head)],
                });
                Ok(head)
            }
            rules::Condition::Of(entity, inner) if *entity == *card => recursion(
                inner,
                card,
                holder,
                legality::PlayDatalog::CARD_VAR,
                candidate,
                holds,
                next_hold,
            ),
            rules::Condition::Of(entity, inner) if *entity == *holder => recursion(
                inner,
                card,
                holder,
                legality::PlayDatalog::PLAYER_VAR,
                candidate,
                holds,
                next_hold,
            ),
            _ => {
                let head = legality::PlayDatalog::holds_head(*next_hold)?;
                *next_hold = next_hold.saturating_add(1);
                let mut body = vec![Literal::Pos(candidate.clone())];
                body.extend(legality::PlayDatalog::translate_condition(
                    condition, card, holder, subject,
                )?);
                holds.push(Rule {
                    head: head.clone(),
                    body,
                });
                Ok(head)
            }
        }
    }
    recursion(
        condition, card, holder, subject, candidate, holds, next_hold,
    )
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::events;
    use crate::keywords;
    use crate::legality;
    use crate::rules;
    use frogmyr::Atom;
    use frogmyr::Literal;
    use frogmyr::Term;
    use frogmyr::Value;

    use super::translate_holds;
    use crate::testing;

    const CARD: rules::ArgRef = rules::ArgRef::Arg(events::Play::CARD);
    const HOLDER: rules::ArgRef = rules::ArgRef::Arg(events::Play::PLAYER);

    #[test]
    fn test_usage() {
        let candidate = testing::play_candidate();
        let mut holds = Vec::new();
        let mut next_hold = 0;
        let root = translate_holds(
            &rules::has_priority(events::Play::PLAYER),
            &CARD,
            &HOLDER,
            legality::PlayDatalog::CARD_VAR,
            &candidate,
            &mut holds,
            &mut next_hold,
        )
        .unwrap();
        assert_eq!(root, legality::PlayDatalog::holds_head(0).unwrap());
        assert_eq!(next_hold, 1);
        assert_eq!(
            holds,
            vec![frogmyr::Rule {
                head: legality::PlayDatalog::holds_head(0).unwrap(),
                body: vec![
                    Literal::Pos(candidate),
                    Literal::Pos(Atom {
                        rel: legality::PlayDatalog::PRIORITY,
                        terms: vec![Term::Var(legality::PlayDatalog::PLAYER_VAR)],
                    }),
                ],
            }]
        );
        let blocker = rules::ArgRef::Arg(events::DeclareBlocker::BLOCKER);
        let attacker = rules::ArgRef::Arg(events::DeclareBlocker::ATTACKER);
        let sky_court = (!rules::has_keyword(keywords::flying().id))
            .of(events::DeclareBlocker::BLOCKER)
            | rules::has_keyword(keywords::flying().id).of(events::DeclareBlocker::ATTACKER);
        let candidate = testing::play_candidate();
        let mut sky_holds = Vec::new();
        let mut sky_next = 0;
        let sky_root = translate_holds(
            &sky_court,
            &blocker,
            &attacker,
            legality::PlayDatalog::CARD_VAR,
            &candidate,
            &mut sky_holds,
            &mut sky_next,
        )
        .unwrap();
        assert_eq!(sky_root, legality::PlayDatalog::holds_head(0).unwrap());
        assert!(
            sky_holds.iter().any(|rule| rule.body.contains(&Literal::Pos(Atom {
                rel: legality::PlayDatalog::HAS_KEYWORD,
                terms: vec![
                    Term::Var(legality::PlayDatalog::PLAYER_VAR),
                    Term::Val(Value::Sym(*keywords::flying().id)),
                ],
            }))),
            "the attacker's keyword is read on the second argument"
        );
        let mut sky_rules = vec![frogmyr::Rule {
            head: candidate.clone(),
            body: vec![Literal::Pos(candidate.clone())],
        }];
        sky_rules.extend(sky_holds);
        let program = frogmyr::Program { rules: sky_rules };
        let mut facts = frogmyr::Facts::default();
        let _ = facts.add(
            legality::PlayDatalog::PLAY_CANDIDATE,
            vec![Value::Sym("ground"), Value::Sym("sky")],
        );
        let _ = facts.add(
            legality::PlayDatalog::PLAY_CANDIDATE,
            vec![Value::Sym("sky"), Value::Sym("ground")],
        );
        let _ = facts.add(
            legality::PlayDatalog::HAS_KEYWORD,
            vec![Value::Sym("sky"), Value::Sym(*keywords::flying().id)],
        );
        let derived = program.evaluate(&facts).unwrap();
        assert!(derived.contains(&sky_root.rel, &[Value::Sym("ground"), Value::Sym("sky")]));
        assert!(!derived.contains(&sky_root.rel, &[Value::Sym("sky"), Value::Sym("ground")]));
    }

    #[test]
    fn all_conjoins_the_children() {
        let candidate = testing::play_candidate();
        let mut holds = Vec::new();
        let mut next_hold = 0;
        let condition =
            rules::has_priority(events::Play::PLAYER) & rules::active_player(events::Play::PLAYER);
        let root = translate_holds(
            &condition,
            &CARD,
            &HOLDER,
            legality::PlayDatalog::CARD_VAR,
            &candidate,
            &mut holds,
            &mut next_hold,
        )
        .unwrap();
        assert_eq!(root, legality::PlayDatalog::holds_head(0).unwrap());
        assert_eq!(next_hold, 3);
        assert_eq!(holds.len(), 3);
        assert_eq!(
            holds[2],
            frogmyr::Rule {
                head: legality::PlayDatalog::holds_head(0).unwrap(),
                body: vec![
                    Literal::Pos(candidate),
                    Literal::Pos(legality::PlayDatalog::holds_head(1).unwrap()),
                    Literal::Pos(legality::PlayDatalog::holds_head(2).unwrap()),
                ],
            }
        );
    }

    #[test]
    fn any_disjoins_the_children() {
        let candidate = testing::play_candidate();
        let mut holds = Vec::new();
        let mut next_hold = 0;
        let condition =
            rules::has_priority(events::Play::PLAYER) | rules::active_player(events::Play::PLAYER);
        let root = translate_holds(
            &condition,
            &CARD,
            &HOLDER,
            legality::PlayDatalog::CARD_VAR,
            &candidate,
            &mut holds,
            &mut next_hold,
        )
        .unwrap();
        assert_eq!(root, legality::PlayDatalog::holds_head(0).unwrap());
        assert_eq!(next_hold, 3);
        assert_eq!(holds.len(), 4);
        assert_eq!(
            holds[1],
            frogmyr::Rule {
                head: legality::PlayDatalog::holds_head(0).unwrap(),
                body: vec![
                    Literal::Pos(candidate.clone()),
                    Literal::Pos(legality::PlayDatalog::holds_head(1).unwrap()),
                ],
            }
        );
        assert_eq!(
            holds[3],
            frogmyr::Rule {
                head: legality::PlayDatalog::holds_head(0).unwrap(),
                body: vec![
                    Literal::Pos(candidate),
                    Literal::Pos(legality::PlayDatalog::holds_head(2).unwrap()),
                ],
            }
        );
    }

    #[test]
    fn not_negates_the_child() {
        let candidate = testing::play_candidate();
        let mut holds = Vec::new();
        let mut next_hold = 0;
        let condition = !rules::has_priority(events::Play::PLAYER);
        let root = translate_holds(
            &condition,
            &CARD,
            &HOLDER,
            legality::PlayDatalog::CARD_VAR,
            &candidate,
            &mut holds,
            &mut next_hold,
        )
        .unwrap();
        assert_eq!(root, legality::PlayDatalog::holds_head(0).unwrap());
        assert_eq!(next_hold, 2);
        assert_eq!(holds.len(), 2);
        assert_eq!(
            holds[1],
            frogmyr::Rule {
                head: legality::PlayDatalog::holds_head(0).unwrap(),
                body: vec![
                    Literal::Pos(candidate),
                    Literal::Neg(legality::PlayDatalog::holds_head(1).unwrap()),
                ],
            }
        );
    }

    #[test]
    fn of_a_compound_holds_when_the_inner_condition_holds() {
        let candidate = testing::play_candidate();
        let mut holds = Vec::new();
        let mut next_hold = 0;
        let condition = (!rules::card_type(cards::CardType::Land)).of(events::Play::CARD);
        let root = translate_holds(
            &condition,
            &CARD,
            &HOLDER,
            legality::PlayDatalog::CARD_VAR,
            &candidate,
            &mut holds,
            &mut next_hold,
        )
        .unwrap();
        assert_eq!(root, legality::PlayDatalog::holds_head(0).unwrap());
        assert_eq!(next_hold, 2);
        assert_eq!(holds.len(), 2);
        assert_eq!(
            holds[0],
            frogmyr::Rule {
                head: legality::PlayDatalog::holds_head(1).unwrap(),
                body: vec![
                    Literal::Pos(candidate.clone()),
                    Literal::Pos(Atom {
                        rel: legality::PlayDatalog::CARD_TYPE,
                        terms: vec![
                            Term::Var(legality::PlayDatalog::CARD_VAR),
                            Term::Val(Value::Sym(legality::PlayDatalog::LAND)),
                        ],
                    }),
                ],
            }
        );
        assert_eq!(
            holds[1],
            frogmyr::Rule {
                head: legality::PlayDatalog::holds_head(0).unwrap(),
                body: vec![
                    Literal::Pos(candidate),
                    Literal::Neg(legality::PlayDatalog::holds_head(1).unwrap()),
                ],
            }
        );
    }
}
