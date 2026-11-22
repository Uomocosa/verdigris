use crate::cards;
use crate::keywords;
use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TapsWhenAttacking;

#[rustfmt::skip]
impl rules::traits::rule_question::RuleQuestion for TapsWhenAttacking {
    type Answer = bool;
    const QUESTION: rules::Question = rules::Question::TapsWhenAttacking;
    fn default(characteristics: &cards::Characteristics) -> bool { !characteristics.keywords.contains(&keywords::Keyword::Vigilance) }
    fn into_answer(answer: bool) -> rules::Answer { rules::Answer::TapsWhenAttacking(answer) }
    fn from_answer(answer: rules::Answer) -> Option<bool> { answer.taps_when_attacking() }
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
        assert!(rules::TapsWhenAttacking::default(&bears_characteristics));
        let vigilant = cards::Characteristics {
            keywords: vec![keywords::Keyword::Vigilance],
            ..bears_characteristics
        };
        assert!(!rules::TapsWhenAttacking::default(&vigilant));
    }
}
