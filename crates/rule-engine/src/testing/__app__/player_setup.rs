use crate::cards;

#[derive(Clone, Debug)]
pub struct PlayerSetup {
    pub(crate) seat: usize,
    pub(crate) hand: Vec<cards::CardDefinition>,
    pub(crate) battlefield: Vec<cards::CardDefinition>,
    pub(crate) tapped_battlefield: Vec<cards::CardDefinition>,
    pub(crate) library: Option<Vec<cards::CardDefinition>>,
    pub(crate) mana_pool: cards::ManaPool,
    pub(crate) life: i32,
    pub(crate) active: bool,
}

impl PlayerSetup {
    #[must_use]
    pub const fn seat(seat: usize) -> Self {
        Self {
            seat,
            hand: Vec::new(),
            battlefield: Vec::new(),
            tapped_battlefield: Vec::new(),
            library: None,
            mana_pool: cards::ManaPool::EMPTY,
            life: 20,
            active: false,
        }
    }
}

impl PlayerSetup {
    #[must_use]
    pub fn hand(
        mut self,
        definitions: impl IntoIterator<Item = impl Into<cards::CardDefinition>>,
    ) -> Self {
        self.hand.extend(definitions.into_iter().map(Into::into));
        self
    }

    #[must_use]
    pub fn battlefield(
        mut self,
        definitions: impl IntoIterator<Item = impl Into<cards::CardDefinition>>,
    ) -> Self {
        self.battlefield
            .extend(definitions.into_iter().map(Into::into));
        self
    }

    #[must_use]
    pub fn battlefield_tapped(
        mut self,
        definitions: impl IntoIterator<Item = impl Into<cards::CardDefinition>>,
    ) -> Self {
        self.tapped_battlefield
            .extend(definitions.into_iter().map(Into::into));
        self
    }

    #[must_use]
    pub fn library(mut self, definitions: impl IntoIterator<Item = cards::CardDefinition>) -> Self {
        self.library = Some(definitions.into_iter().collect());
        self
    }

    #[must_use]
    pub const fn mana_pool(mut self, pool: cards::ManaPool) -> Self {
        self.mana_pool = pool;
        self
    }

    #[must_use]
    pub const fn life(mut self, amount: i32) -> Self {
        self.life = amount;
        self
    }

    #[must_use]
    pub const fn active(mut self) -> Self {
        self.active = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::HAND;
    use crate::cards;
    use crate::mana;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let setup = P3
            .life(7)
            .mana_pool(mana![2, U])
            .hand([DIVINATION])
            .battlefield([GRIZZLY_BEARS])
            .library([cards::CardDefinition::from(MURDER)]);

        assert_eq!(setup.seat, 3);
        assert_eq!(setup.life, 7);
        assert_eq!(setup.mana_pool, mana![2, U]);
        assert_eq!(setup.hand, vec![cards::CardDefinition::from(DIVINATION)]);
        assert_eq!(
            setup.battlefield,
            vec![cards::CardDefinition::from(GRIZZLY_BEARS)]
        );
        assert_eq!(
            setup.library,
            Some(vec![cards::CardDefinition::from(MURDER)])
        );
        assert!(!setup.active);
    }

    #[test]
    fn card_constant_name_matches_its_definition() {
        let fixtures = [
            CANCEL,
            DARKSTEEL_MYR,
            DISPEL,
            DIVINATION,
            DOOM_BLADE,
            ESSENCE_SCATTER,
            FOREST,
            GIANT_GROWTH,
            GRIZZLY_BEARS,
            LIGHTNING_BOLT,
            MIND_ROT,
            MURDER,
            NEGATE,
            SIGN_IN_BLOOD,
            TYPHOID_RATS,
            WALKING_CORPSE,
        ];

        for card in fixtures {
            assert_eq!(*((card.definition)()).name, *card.name);
        }
    }

    #[test]
    fn hand_places_a_card_constant_in_the_hand_zone() {
        let (game, [p1, _]) = MockGame::new([P1.hand([LIGHTNING_BOLT]), P2]);

        assert!(game.player(p1).cards_in(HAND).has(LIGHTNING_BOLT));
    }
}
