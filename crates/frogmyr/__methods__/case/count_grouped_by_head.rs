use crate::Atom;
use crate::Case;
use crate::CmpOp;
use crate::Facts;
use crate::Literal;
use crate::Program;
use crate::Rel;
use crate::Rule;
use crate::Term;
use crate::Value;

pub fn count_grouped_by_head() -> Case {
    let player = Rel("player");
    let controller = Rel("controller");
    let card_type = Rel("card_type");
    let creature_count = Rel("creature_count");
    let too_many = Rel("too_many");
    let first_player = Value::Entity(1);
    let second_player = Value::Entity(2);
    let first_card = Value::Entity(10);
    let second_card = Value::Entity(11);
    let third_card = Value::Entity(12);
    let land_card = Value::Entity(13);
    let creature = Value::Sym("creature");
    let mut facts = Facts::default();
    let _ = facts.add(player, vec![first_player]);
    let _ = facts.add(player, vec![second_player]);
    let _ = facts.add(controller, vec![first_card, first_player]);
    let _ = facts.add(controller, vec![second_card, first_player]);
    let _ = facts.add(controller, vec![third_card, second_player]);
    let _ = facts.add(controller, vec![land_card, first_player]);
    let _ = facts.add(card_type, vec![first_card, creature]);
    let _ = facts.add(card_type, vec![second_card, creature]);
    let _ = facts.add(card_type, vec![third_card, creature]);
    let _ = facts.add(card_type, vec![land_card, Value::Sym("land")]);
    let mut expected = facts.clone();
    let _ = expected.add(creature_count, vec![first_player, Value::Int(2)]);
    let _ = expected.add(creature_count, vec![second_player, Value::Int(1)]);
    let _ = expected.add(too_many, vec![first_player]);
    Case {
        name: "count_grouped_by_head",
        program: Program {
            rules: vec![
                Rule {
                    head: Atom {
                        rel: creature_count,
                        terms: vec![Term::Var("P"), Term::Var("N")],
                    },
                    body: vec![Literal::Count {
                        output: "N",
                        body: vec![
                            Atom {
                                rel: controller,
                                terms: vec![Term::Var("C"), Term::Var("P")],
                            },
                            Atom {
                                rel: card_type,
                                terms: vec![Term::Var("C"), Term::Val(creature)],
                            },
                        ],
                    }],
                },
                Rule {
                    head: Atom {
                        rel: too_many,
                        terms: vec![Term::Var("P")],
                    },
                    body: vec![
                        Literal::Pos(Atom {
                            rel: creature_count,
                            terms: vec![Term::Var("P"), Term::Var("N")],
                        }),
                        Literal::Cmp(Term::Var("N"), CmpOp::Ge, Term::Val(Value::Int(2))),
                    ],
                },
            ],
        },
        facts,
        expected,
    }
}

#[cfg(test)]
mod tests {
    use super::count_grouped_by_head;

    #[test]
    fn test_usage() {
        let case = count_grouped_by_head();
        assert_eq!(case.name, "count_grouped_by_head");
        assert_eq!(case.program.rules.len(), 2);
    }
}
