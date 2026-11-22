use crate::cards;
use crate::cost;
use crate::effects;
use crate::pt;
use crate::testing;

pub const SOLDIER_TOKEN: testing::Card = testing::Card {
    name: testing::TestCardName("Soldier"),
    definition: || cards::creature("Soldier", cost![W], pt!(1, 1)),
};

pub const RAISE_THE_ALARM: testing::Card = testing::Card {
    name: testing::TestCardName("Raise the Alarm"),
    definition: || {
        cards::instant("Raise the Alarm", cost![1, W])
            .spell(effects::creates(2, SOLDIER_TOKEN.definition.into()))
    },
};

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::effects;
    use crate::pt;

    use super::{RAISE_THE_ALARM, SOLDIER_TOKEN};

    #[test]
    fn test_usage() {
        let soldier = (SOLDIER_TOKEN.definition)();
        assert_eq!(*soldier.name, "Soldier");
        assert_eq!(soldier.types, vec![cards::CardType::Creature]);
        assert_eq!(soldier.power_toughness, Some(pt!(1, 1)));
        assert_eq!(soldier.colors(), vec![cards::Color::White]);

        let alarm = (RAISE_THE_ALARM.definition)();
        assert_eq!(*alarm.name, "Raise the Alarm");
        assert_eq!(alarm.types, vec![cards::CardType::Instant]);
        assert_eq!(alarm.cost, cost![1, W]);
        assert_eq!(
            alarm.effect,
            Some(effects::creates(2, SOLDIER_TOKEN.definition.into()))
        );
    }
}
