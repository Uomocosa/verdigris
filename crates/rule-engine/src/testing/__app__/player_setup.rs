use crate::cards;

#[derive(Clone, Debug)]
pub struct PlayerSetup {
    pub(crate) seat: usize,
    pub(crate) hand: Vec<cards::CardDefinition>,
    pub(crate) battlefield: Vec<cards::CardDefinition>,
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
            library: None,
            mana_pool: cards::ManaPool::EMPTY,
            life: 20,
            active: false,
        }
    }
}

impl PlayerSetup {
    #[must_use]
    pub fn hand(mut self, definitions: impl IntoIterator<Item = cards::CardDefinition>) -> Self {
        self.hand.extend(definitions);
        self
    }

    #[must_use]
    pub fn battlefield(
        mut self,
        definitions: impl IntoIterator<Item = cards::CardDefinition>,
    ) -> Self {
        self.battlefield.extend(definitions);
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
    use crate::mana;
    use crate::testing::*;

    #[test]
    fn test_usage() {
        let setup = P3.life(7).mana_pool(mana![2, U]).hand([divination()]);

        assert_eq!(setup.seat, 3);
        assert_eq!(setup.life, 7);
        assert_eq!(setup.mana_pool, mana![2, U]);
        assert_eq!(setup.hand, vec![divination()]);
        assert_eq!(setup.battlefield, vec![]);
        assert!(setup.library.is_none());
        assert!(!setup.active);
    }
}
