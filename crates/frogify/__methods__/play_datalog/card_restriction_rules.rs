use frogmyr::Atom;
use frogmyr::Literal;
use frogmyr::Rule;
use frogmyr::Term;
use frogmyr::Value;
use verdigris_rule_engine::rules;
use verdigris_rule_engine::rules::traits::event::Event;

use crate::PlayDatalog;
use crate::TranslationError;

pub fn card_restriction_rules<E: Event>(
    answers: &rules::Rules,
    event: &E,
    candidate: &Atom,
    card: &rules::ArgRef,
    holder: &rules::ArgRef,
    event_name: &'static str,
    rules: &mut Vec<Rule>,
    next_hold: &mut usize,
) -> Result<(), TranslationError> {
    for change in PlayDatalog::holder_changes(answers, event, rules::Target::event::<E>()) {
        let rules::Change::Restrict { restriction, .. } = change else {
            continue;
        };
        let mut scratch = Vec::new();
        let mut scratch_next = *next_hold;
        let root = match PlayDatalog::translate_holds(
            &restriction.condition.bind(event),
            card,
            holder,
            PlayDatalog::CARD_VAR,
            candidate,
            &mut scratch,
            &mut scratch_next,
        ) {
            Ok(root) => root,
            Err(TranslationError::Untranslatable(_) | TranslationError::TooManyConditions(_)) => {
                continue;
            }
            Err(engine) => return Err(engine),
        };
        rules.append(&mut scratch);
        *next_hold = scratch_next;
        let mut body = vec![Literal::Pos(candidate.clone()), Literal::Neg(root)];
        if !restriction.unwaivable {
            body.push(Literal::Neg(Atom {
                rel: PlayDatalog::WAIVED,
                terms: vec![
                    Term::Val(Value::Sym(event_name)),
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Var(PlayDatalog::PLAYER_VAR),
                    Term::Val(Value::Sym(*restriction.id)),
                ],
            }));
            PlayDatalog::waiver_rules(
                answers,
                event,
                restriction.id,
                candidate,
                card,
                holder,
                event_name,
                &mut *rules,
                &mut *next_hold,
            )?;
        }
        rules.push(Rule {
            head: Atom {
                rel: PlayDatalog::VIOLATES,
                terms: vec![
                    Term::Val(Value::Sym(event_name)),
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Var(PlayDatalog::PLAYER_VAR),
                    Term::Val(Value::Sym(*restriction.id)),
                ],
            },
            body,
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use verdigris_rule_engine::BATTLEFIELD;
    use verdigris_rule_engine::events;
    use verdigris_rule_engine::rules;
    use verdigris_rule_engine::testing::GRIZZLY_BEARS;
    use verdigris_rule_engine::testing::MockGame;
    use verdigris_rule_engine::testing::P1;
    use verdigris_rule_engine::testing::P2;
    use verdigris_rule_engine::testing::WIND_DRAKE;

    use super::card_restriction_rules;
    use crate::BlockDatalog;
    use crate::testing;

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([WIND_DRAKE]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [drake] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(WIND_DRAKE).unwrap();
        let [bears] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GRIZZLY_BEARS).unwrap();
        game.read_rules(|answers, _legal| {
            let event = events::DeclareBlocker {
                blocker: *bears,
                attacker: *drake,
            };
            let mut added = Vec::new();
            let mut next_hold = 0;
            card_restriction_rules(
                answers,
                &event,
                &testing::block_candidate(),
                &rules::ArgRef::Arg(events::DeclareBlocker::BLOCKER),
                &rules::ArgRef::Arg(events::DeclareBlocker::ATTACKER),
                BlockDatalog::BLOCK,
                &mut added,
                &mut next_hold,
            )
            .unwrap();
            assert!(next_hold > 0);
            assert_eq!(
                added.iter().filter(|rule| *rule.head.rel == "violates").count(),
                1,
                "the drake's flying is the only card restriction on this pair"
            );
        })
        .unwrap();
    }
}
