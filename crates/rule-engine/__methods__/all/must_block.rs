use crate::abilities;
use crate::effects;
use crate::rules;
use crate::statics;

pub fn must_block(all: statics::All) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        change: effects::Change::Rule {
            question: rules::Question::MustBlock,
            answer: rules::Answer::MustBlock(true),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::rules;
    use crate::statics;

    use super::must_block;

    #[test]
    fn test_usage() {
        let ability = must_block(statics::all(CREATURE));
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.change,
            effects::Change::Rule {
                question: rules::Question::MustBlock,
                answer: rules::Answer::MustBlock(true),
            }
        );
    }
}
