use crate::cards;
use crate::keywords;
use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MustBlock;

#[rustfmt::skip]
impl rules::traits::rule_question::RuleQuestion for MustBlock {
    type Answer = bool;
    const QUESTION: rules::Question = rules::Question::MustBlock;
    fn default(characteristics: &cards::Characteristics) -> bool { characteristics.keywords.contains(&keywords::Keyword::MustBlockIfAble) }
    fn into_answer(answer: bool) -> rules::Answer { rules::Answer::MustBlock(answer) }
    fn from_answer(answer: rules::Answer) -> Option<bool> { answer.must_block() }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::keywords;
    use crate::pt;
    use crate::rules;
    use crate::rules::traits::rule_question::RuleQuestion;

    #[test]
    fn test_usage() {
        let bears = cards::creature("Grizzly Bears", cost![1, G], pt!(2, 2));
        let bears_characteristics = cards::Characteristics::from_printed(&bears, true);
        assert!(!rules::MustBlock::default(&bears_characteristics));
        let forced = cards::Characteristics {
            keywords: vec![keywords::Keyword::MustBlockIfAble],
            ..bears_characteristics
        };
        assert!(rules::MustBlock::default(&forced));
    }
}
