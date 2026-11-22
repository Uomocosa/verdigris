use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn must_attack(all: statics::All) -> abilities::Static {
    abilities::Static {
        filter: *all,
        change: effects::Change::Rule {
            question: rules::Question::MustAttack,
            answer: rules::Answer::MustAttack(true),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::statics;

    use super::must_attack;

    #[test]
    fn test_usage() {
        let ability = must_attack(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.change,
            effects::Change::Rule {
                question: rules::Question::MustAttack,
                answer: rules::Answer::MustAttack(true),
            }
        );
    }
}
