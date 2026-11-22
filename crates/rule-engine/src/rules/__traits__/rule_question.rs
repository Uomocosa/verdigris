use crate::cards;
use crate::rules;

pub trait RuleQuestion: Copy {
    type Answer: Copy;
    const QUESTION: rules::Question;
    const COMBINE: rules::Combine = rules::Combine::LatestTimestamp;
    fn default(characteristics: &cards::Characteristics) -> Self::Answer;
    fn into_answer(answer: Self::Answer) -> rules::Answer;
    fn from_answer(answer: rules::Answer) -> Option<Self::Answer>;
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::rules::traits::rule_question::RuleQuestion;

    #[test]
    fn test_usage() {
        assert_eq!(
            rules::TapsWhenAttacking::QUESTION,
            rules::Question::TapsWhenAttacking
        );
        assert_eq!(
            rules::TapsWhenAttacking::COMBINE,
            rules::Combine::LatestTimestamp
        );
        let answer = rules::TapsWhenAttacking::into_answer(true);
        assert_eq!(answer, rules::Answer::TapsWhenAttacking(true));
        assert_eq!(rules::TapsWhenAttacking::from_answer(answer), Some(true));
    }
}
