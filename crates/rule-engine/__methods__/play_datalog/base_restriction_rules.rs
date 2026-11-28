use crate::rules;
use crate::rules::traits::event::Event;
use frogmyr::Atom;
use frogmyr::Literal;
use frogmyr::Rule;
use frogmyr::Term;
use frogmyr::Value;

use crate::legality;

pub fn base_restriction_rules<E: Event>(
    answers: &rules::Rules,
    event: &E,
    candidate: &Atom,
    card: &rules::ArgRef,
    holder: &rules::ArgRef,
    event_name: &'static str,
    rules: &mut Vec<Rule>,
    next_hold: &mut usize,
) -> Result<(), legality::TranslationError> {
    for restriction in E::restrictions() {
        let root = legality::PlayDatalog::translate_holds(
            &restriction.condition,
            card,
            holder,
            legality::PlayDatalog::CARD_VAR,
            candidate,
            rules,
            next_hold,
        )?;
        let mut body = vec![Literal::Pos(candidate.clone()), Literal::Neg(root)];
        if !restriction.unwaivable {
            body.push(Literal::Neg(Atom {
                rel: legality::PlayDatalog::WAIVED,
                terms: vec![
                    Term::Val(Value::Sym(event_name)),
                    Term::Var(legality::PlayDatalog::CARD_VAR),
                    Term::Var(legality::PlayDatalog::PLAYER_VAR),
                    Term::Val(Value::Sym(*restriction.id)),
                ],
            }));
            legality::PlayDatalog::waiver_rules(
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
                rel: legality::PlayDatalog::VIOLATES,
                terms: vec![
                    Term::Val(Value::Sym(event_name)),
                    Term::Var(legality::PlayDatalog::CARD_VAR),
                    Term::Var(legality::PlayDatalog::PLAYER_VAR),
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
    use crate::HAND;
    use crate::events;
    use crate::legality;
    use crate::players;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::testing::FOREST;
    use crate::testing::MockGame;
    use crate::testing::P1;
    use crate::testing::P2;

    use super::base_restriction_rules;
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
                legality::PlayDatalog::PLAY,
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
