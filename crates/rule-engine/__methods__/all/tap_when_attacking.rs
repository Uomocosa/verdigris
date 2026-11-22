use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn tap_when_attacking(all: statics::All) -> abilities::Static {
    abilities::Static {
        filter: *all,
        change: effects::Change::Rule {
            question: rules::Question::TapsWhenAttacking,
            answer: rules::Answer::TapsWhenAttacking(true),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::statics;

    use super::tap_when_attacking;

    #[test]
    fn test_usage() {
        let ability = tap_when_attacking(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.change,
            effects::Change::Rule {
                question: rules::Question::TapsWhenAttacking,
                answer: rules::Answer::TapsWhenAttacking(true),
            }
        );
    }
}
