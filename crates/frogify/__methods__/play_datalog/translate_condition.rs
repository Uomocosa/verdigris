use frogmyr::Atom;
use frogmyr::Literal;
use frogmyr::Term;
use frogmyr::Value;
use verdigris_rule_engine::cards;
use verdigris_rule_engine::rules;
use verdigris_rule_engine::zones;

use crate::PlayDatalog;
use crate::TranslationError;

pub fn translate_condition(
    condition: &rules::Condition,
    card: &rules::ArgRef,
    holder: &rules::ArgRef,
    subject: &'static str,
) -> Result<Vec<Literal>, TranslationError> {
    if let Some(literals) = PlayDatalog::translate_subject_condition(condition, subject)? {
        return Ok(literals);
    }
    if *condition == rules::main_phase() {
        return Ok(vec![
            Literal::Pos(Atom {
                rel: PlayDatalog::STEP,
                terms: vec![Term::Var("S")],
            }),
            Literal::Pos(Atom {
                rel: PlayDatalog::MAIN_STEP,
                terms: vec![Term::Var("S")],
            }),
        ]);
    }
    if *condition == rules::stack_empty() {
        return Ok(vec![Literal::Pos(Atom {
            rel: PlayDatalog::STACK_EMPTY,
            terms: vec![Term::Var(PlayDatalog::PLAYER_VAR)],
        })]);
    }
    let is_holder = |reference: &rules::ArgRef| *reference == *holder;
    let positive = |relation: frogmyr::Rel, terms: Vec<Term>| {
        vec![Literal::Pos(Atom {
            rel: relation,
            terms,
        })]
    };
    match condition {
        rules::Condition::HasPriority(reference) if is_holder(reference) => Ok(positive(
            PlayDatalog::PRIORITY,
            vec![Term::Var(PlayDatalog::PLAYER_VAR)],
        )),
        rules::Condition::ActivePlayer(reference) if is_holder(reference) => Ok(positive(
            PlayDatalog::ACTIVE,
            vec![Term::Var(PlayDatalog::PLAYER_VAR)],
        )),
        rules::Condition::LandPlayAvailable(reference) if is_holder(reference) => Ok(positive(
            PlayDatalog::LAND_PLAY_AVAILABLE,
            vec![Term::Var(PlayDatalog::PLAYER_VAR)],
        )),
        rules::Condition::Of(entity, inner) if *entity == *card => {
            translate_condition(inner, card, holder, PlayDatalog::CARD_VAR)
                .map_err(|_| TranslationError::Untranslatable(condition.clone()))
        }
        rules::Condition::Of(entity, inner) if *entity == *holder => {
            translate_condition(inner, card, holder, PlayDatalog::PLAYER_VAR)
                .map_err(|_| TranslationError::Untranslatable(condition.clone()))
        }
        rules::Condition::True => Ok(Vec::new()),
        rules::Condition::InZone(zone, zone_holder) if is_holder(zone_holder) => {
            let name = match *zone {
                zones::Zone::Library => "Library",
                zones::Zone::Hand => "Hand",
                zones::Zone::Battlefield => "Battlefield",
                zones::Zone::Graveyard => "Graveyard",
                zones::Zone::Exile => "Exile",
                zones::Zone::Stack => "Stack",
            };
            Ok(positive(
                PlayDatalog::IN_ZONE,
                vec![
                    Term::Var(subject),
                    Term::Val(Value::Sym(name)),
                    Term::Var(PlayDatalog::PLAYER_VAR),
                ],
            ))
        }
        rules::Condition::PointsTo(component, owner)
            if *component == rules::ComponentRef::relation::<cards::Owner>()
                && is_holder(owner) =>
        {
            Ok(positive(
                PlayDatalog::OWNER,
                vec![Term::Var(subject), Term::Var(PlayDatalog::PLAYER_VAR)],
            ))
        }
        _ => Err(TranslationError::Untranslatable(condition.clone())),
    }
}

#[cfg(test)]
mod tests {
    use frogmyr::Atom;
    use frogmyr::Literal;
    use frogmyr::Term;
    use frogmyr::Value;
    use verdigris_rule_engine::cards;
    use verdigris_rule_engine::events;
    use verdigris_rule_engine::rules;
    use verdigris_rule_engine::turns;
    use verdigris_rule_engine::zones;

    use super::translate_condition;
    use crate::PlayDatalog;
    use crate::TranslationError;

    #[test]
    fn test_usage() {
        let card = rules::ArgRef::Arg(events::Play::CARD);
        let holder = rules::ArgRef::Arg(events::Play::PLAYER);
        assert_eq!(
            translate_condition(
                &rules::has_priority(events::Play::PLAYER),
                &card,
                &holder,
                PlayDatalog::CARD_VAR
            ),
            Ok(vec![Literal::Pos(Atom {
                rel: PlayDatalog::PRIORITY,
                terms: vec![Term::Var(PlayDatalog::PLAYER_VAR)],
            })])
        );
        assert_eq!(
            translate_condition(&rules::stack_empty(), &card, &holder, PlayDatalog::CARD_VAR),
            Ok(vec![Literal::Pos(Atom {
                rel: PlayDatalog::STACK_EMPTY,
                terms: vec![Term::Var(PlayDatalog::PLAYER_VAR)],
            })])
        );
        assert_eq!(
            translate_condition(&rules::main_phase(), &card, &holder, PlayDatalog::CARD_VAR),
            Ok(vec![
                Literal::Pos(Atom {
                    rel: PlayDatalog::STEP,
                    terms: vec![Term::Var("S")],
                }),
                Literal::Pos(Atom {
                    rel: PlayDatalog::MAIN_STEP,
                    terms: vec![Term::Var("S")],
                }),
            ])
        );
        let always = rules::Condition::True;
        assert_eq!(
            translate_condition(&always, &card, &holder, PlayDatalog::CARD_VAR),
            Ok(Vec::new())
        );
        let graveyard = rules::in_zone(zones::Zone::Graveyard, events::Play::PLAYER);
        assert_eq!(
            translate_condition(&graveyard, &card, &holder, PlayDatalog::CARD_VAR),
            Ok(vec![Literal::Pos(Atom {
                rel: PlayDatalog::IN_ZONE,
                terms: vec![
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Val(Value::Sym("Graveyard")),
                    Term::Var(PlayDatalog::PLAYER_VAR),
                ],
            })])
        );
    }

    #[test]
    fn card_types_other_than_land_translate() {
        let card = rules::ArgRef::Arg(events::Play::CARD);
        let holder = rules::ArgRef::Arg(events::Play::PLAYER);
        assert_eq!(
            translate_condition(
                &rules::card_type(cards::CardType::Instant).of(events::Play::CARD),
                &card,
                &holder,
                PlayDatalog::CARD_VAR
            ),
            Ok(vec![Literal::Pos(Atom {
                rel: PlayDatalog::CARD_TYPE,
                terms: vec![
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Val(Value::Sym("Instant")),
                ],
            })])
        );
    }

    #[test]
    fn an_untranslatable_atom_is_an_explicit_error() {
        let card = rules::ArgRef::Arg(events::Play::CARD);
        let holder = rules::ArgRef::Arg(events::Play::PLAYER);
        let attacking = rules::has::<cards::Attacking>().of(events::Play::CARD);
        assert_eq!(
            translate_condition(&attacking, &card, &holder, PlayDatalog::CARD_VAR),
            Err(TranslationError::Untranslatable(attacking.clone()))
        );
        assert_eq!(
            translate_condition(
                &rules::step_is(turns::Step::Upkeep),
                &card,
                &holder,
                PlayDatalog::CARD_VAR
            ),
            Err(TranslationError::Untranslatable(rules::step_is(
                turns::Step::Upkeep
            )))
        );
    }

    #[test]
    fn compounds_are_not_atoms() {
        let card = rules::ArgRef::Arg(events::Play::CARD);
        let holder = rules::ArgRef::Arg(events::Play::PLAYER);
        let both =
            rules::has_priority(events::Play::PLAYER) & rules::active_player(events::Play::PLAYER);
        assert_eq!(
            translate_condition(&both, &card, &holder, PlayDatalog::CARD_VAR),
            Err(TranslationError::Untranslatable(both.clone()))
        );
        let either =
            rules::has_priority(events::Play::PLAYER) | rules::active_player(events::Play::PLAYER);
        assert_eq!(
            translate_condition(&either, &card, &holder, PlayDatalog::CARD_VAR),
            Err(TranslationError::Untranslatable(either.clone()))
        );
        let negated = !rules::has_priority(events::Play::PLAYER);
        assert_eq!(
            translate_condition(&negated, &card, &holder, PlayDatalog::CARD_VAR),
            Err(TranslationError::Untranslatable(negated.clone()))
        );
    }

    #[test]
    fn other_arg_refs_do_not_match() {
        let card = rules::ArgRef::Arg(events::Play::CARD);
        let holder = rules::ArgRef::Arg(events::Play::PLAYER);
        let foreign = rules::has_priority(rules::ArgId("other"));
        assert_eq!(
            translate_condition(&foreign, &card, &holder, PlayDatalog::CARD_VAR),
            Err(TranslationError::Untranslatable(foreign.clone()))
        );
    }

    #[test]
    fn of_a_foreign_card_does_not_match() {
        let card = rules::ArgRef::Arg(events::Play::CARD);
        let holder = rules::ArgRef::Arg(events::Play::PLAYER);
        let foreign_card = rules::ArgRef::Arg(rules::ArgId("other"));
        let condition = rules::card_type(cards::CardType::Land).of(foreign_card);
        assert_eq!(
            translate_condition(&condition, &card, &holder, PlayDatalog::CARD_VAR),
            Err(TranslationError::Untranslatable(condition.clone()))
        );
    }
}
