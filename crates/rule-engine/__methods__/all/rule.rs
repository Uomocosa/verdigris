use crate::abilities;
use crate::effects;
use crate::rules::traits::rule_question::RuleQuestion;
use crate::statics;

pub fn rule<Q: RuleQuestion>(all: statics::All, answer: Q::Answer) -> abilities::Static {
    abilities::Static {
        filter: *all,
        change: effects::Change::Rule {
            question: Q::QUESTION,
            answer: Q::into_answer(answer),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::statics;

    use super::rule;

    #[test]
    fn test_usage() {
        let ability = rule::<rules::TapsWhenAttacking>(statics::all(CREATURE), false);
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.change,
            effects::Change::Rule {
                question: rules::Question::TapsWhenAttacking,
                answer: rules::Answer::TapsWhenAttacking(false),
            }
        );
    }
}
