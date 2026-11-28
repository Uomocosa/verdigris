use frogmyr::Atom;
use frogmyr::Literal;
use frogmyr::Rule;
use frogmyr::Term;
use frogmyr::Value;
use verdigris_rule_engine::rules;
use verdigris_rule_engine::rules::traits::event::Event;

use crate::PlayDatalog;
use crate::TranslationError;

pub fn base_restriction_rules<E: Event>(
    answers: &rules::Rules,
    event: &E,
    candidate: &Atom,
    card: &rules::ArgRef,
    holder: &rules::ArgRef,
    event_name: &'static str,
    rules: &mut Vec<Rule>,
    next_hold: &mut usize,
) -> Result<(), TranslationError> {
    for restriction in E::restrictions() {
        let root = PlayDatalog::translate_holds(
            &restriction.condition,
            card,
            holder,
            PlayDatalog::CARD_VAR,
            candidate,
            rules,
            next_hold,
        )?;
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
    use verdigris_rule_engine::HAND;
    use verdigris_rule_engine::events;
    use verdigris_rule_engine::players;
    use verdigris_rule_engine::rules;
    use verdigris_rule_engine::rules::traits::event::Event;
    use verdigris_rule_engine::testing::FOREST;
    use verdigris_rule_engine::testing::MockGame;
    use verdigris_rule_engine::testing::P1;
    use verdigris_rule_engine::testing::P2;

    use super::base_restriction_rules;
    use crate::PlayDatalog;
    use crate::testing;

    #[test]
    fn test_usage() {
        let (mut game, [p1, _p2]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST).unwrap();
        game.read_rules(|answers, _legal| {
            let event = events::Play {
                card: *forest,
                player: players::PlayerId(*p1),
            };
            let mut added = Vec::new();
            let mut next_hold = 0;
            base_restriction_rules(
                answers,
                &event,
                &testing::play_candidate(),
                &rules::ArgRef::Arg(events::Play::CARD),
                &rules::ArgRef::Arg(events::Play::PLAYER),
                PlayDatalog::PLAY,
                &mut added,
                &mut next_hold,
            )
            .unwrap();
            assert_eq!(
                added.iter().filter(|rule| *rule.head.rel == "violates").count(),
                events::Play::restrictions().len(),
                "one violates rule per base Play restriction"
            );
            assert_eq!(
                added.iter().filter(|rule| rule.head.rel.starts_with("holds_")).count(),
                8,
                "one holds rule per single-atom Play restriction"
            );
            assert!(
                added.iter().all(|rule| *rule.head.rel != "waived"),
                "a vanilla state holds no waivers"
            );
            assert_eq!(next_hold, 8);
        })
        .unwrap();
    }
}
