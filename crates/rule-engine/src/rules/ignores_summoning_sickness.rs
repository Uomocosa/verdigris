use crate::cards;
use crate::keywords;
use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgnoresSummoningSickness;

#[rustfmt::skip]
impl rules::traits::rule_question::RuleQuestion for IgnoresSummoningSickness {
    type Answer = bool;
    const QUESTION: rules::Question = rules::Question::IgnoresSummoningSickness;
    fn default(characteristics: &cards::Characteristics) -> bool { characteristics.keywords.contains(&keywords::StaticKeyword::Haste) }
    fn into_answer(answer: bool) -> rules::Answer { rules::Answer::IgnoresSummoningSickness(answer) }
    fn from_answer(answer: rules::Answer) -> Option<bool> { answer.ignores_summoning_sickness() }
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
        assert!(!rules::IgnoresSummoningSickness::default(
            &bears_characteristics
        ));
        let hasty = cards::Characteristics {
            keywords: vec![keywords::StaticKeyword::Haste],
            ..bears_characteristics
        };
        assert!(rules::IgnoresSummoningSickness::default(&hasty));
    }
}
