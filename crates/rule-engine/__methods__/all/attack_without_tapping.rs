use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn attack_without_tapping(all: statics::All) -> abilities::Static {
    abilities::Static {
        filter: *all,
        change: effects::Change::Rule {
            question: rules::Question::TapsWhenAttacking,
            answer: rules::Answer::TapsWhenAttacking(false),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::statics;

    use super::attack_without_tapping;

    #[test]
    fn test_usage() {
        let ability = attack_without_tapping(statics::all(CREATURE));
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
