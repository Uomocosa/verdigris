use crate::rules;
use crate::rules::traits::event::Event;
use frogmyr::Atom;
use frogmyr::Literal;
use frogmyr::Rule;
use frogmyr::Term;
use frogmyr::Value;

use crate::legality;

pub fn waiver_rules<E: Event>(
    answers: &rules::Rules,
    event: &E,
    id: rules::RestrictionId,
    candidate: &Atom,
    card: &rules::ArgRef,
    holder: &rules::ArgRef,
    event_name: &'static str,
    rules: &mut Vec<Rule>,
    next_hold: &mut usize,
) -> Result<(), legality::TranslationError> {
    for change in
        legality::PlayDatalog::holder_changes(answers, event, rules::Target::Restriction(id))
    {
        match change {
            rules::Change::AsThough { .. } => {
                rules.push(Rule {
                    head: Atom {
                        rel: legality::PlayDatalog::WAIVED,
                        terms: vec![
                            Term::Val(Value::Sym(event_name)),
                            Term::Var(legality::PlayDatalog::CARD_VAR),
                            Term::Var(legality::PlayDatalog::PLAYER_VAR),
                            Term::Val(Value::Sym(*id)),
                        ],
                    },
                    body: vec![Literal::Pos(candidate.clone())],
                });
            }
            rules::Change::Allow { when, .. } => {
                let mut scratch = Vec::new();
                let mut scratch_next = *next_hold;
                let root = match legality::PlayDatalog::translate_holds(
                    &when.bind(event),
                    card,
                    holder,
                    legality::PlayDatalog::CARD_VAR,
                    candidate,
                    &mut scratch,
                    &mut scratch_next,
                ) {
                    Ok(root) => root,
                    Err(
                        legality::TranslationError::Untranslatable(_)
                        | legality::TranslationError::TooManyConditions(_),
                    ) => {
                        continue;
                    }
                    Err(engine) => return Err(engine),
                };
                rules.append(&mut scratch);
                *next_hold = scratch_next;
                rules.push(Rule {
                    head: Atom {
                        rel: legality::PlayDatalog::WAIVED,
                        terms: vec![
                            Term::Val(Value::Sym(event_name)),
                            Term::Var(legality::PlayDatalog::CARD_VAR),
                            Term::Var(legality::PlayDatalog::PLAYER_VAR),
                            Term::Val(Value::Sym(*id)),
                        ],
                    },
                    body: vec![Literal::Pos(candidate.clone()), Literal::Pos(root)],
                });
            }
            _ => {}
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::events;
    use crate::legality;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::testing::GRIZZLY_BEARS;
    use crate::testing::MockGame;
    use crate::testing::P1;
    use crate::testing::P2;
    use crate::testing::TEST_BATTLE_FERVOR;

    use super::waiver_rules;
    use crate::testing;

    #[test]
    fn test_usage() {
        let (mut game, [p1, _p2]) =
            MockGame::new([P1.battlefield([TEST_BATTLE_FERVOR, GRIZZLY_BEARS]), P2]);
        let [bears] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.pass_priority();
        game.read_rules(|answers, _legal| {
            let event = events::Attack { attacker: *bears };
            let mut added = Vec::new();
            let mut next_hold = 0;
            waiver_rules(
                answers,
                &event,
                events::attack::SUMMONING_SICK,
                &testing::attack_candidate(),
                &rules::ArgRef::Arg(events::Attack::ATTACKER),
                &rules::ArgRef::Arg(events::Attack::ATTACKER),
                legality::AttackDatalog::ATTACK,
                &mut added,
                &mut next_hold,
            )
            .unwrap();
            assert_eq!(
                added.iter().filter(|rule| *rule.head.rel == "waived").count(),
                1,
                "the fervor's as-though waiver is the only waiver of summoning sickness here"
            );
            assert_eq!(next_hold, 0, "an unconditional waiver adds no holds rules");
            assert_eq!(
                events::Attack::restrictions().len(),
                4,
                "Attack gained a restriction; this waiver may need another id"
            );
        })
        .unwrap();
    }
}
