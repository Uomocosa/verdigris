use crate::card;
use crate::cards;
use crate::cost;
use crate::keywords;
use crate::pt;
use crate::testing;

pub const CHILD_OF_NIGHT: testing::Card = testing::Card {
    name: testing::TestCardName("Child of Night"),
    definition: || {
        card! {
            cards::creature("Child of Night", cost![1, B], pt!(2, 1));
            keywords::lifelink(),
        }
    },
};

#[cfg(test)]
mod tests {
    use crate::cost;
    use crate::keywords;
    use crate::pt;

    use super::CHILD_OF_NIGHT;

    #[test]
    fn test_usage() {
        let vampire = (CHILD_OF_NIGHT.definition)();
        assert_eq!(*vampire.name, "Child of Night");
        assert_eq!(vampire.cost, cost![1, B]);
        assert_eq!(vampire.power_toughness, Some(pt!(2, 1)));
        assert_eq!(
            vampire.keywords().collect::<Vec<_>>(),
            vec![keywords::StaticKeyword::Lifelink]
        );
    }
}
