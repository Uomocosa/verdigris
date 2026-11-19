use bevy::prelude::Component;

use crate::cards;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ManaPool {
    pub white: u32,
    pub blue: u32,
    pub black: u32,
    pub red: u32,
    pub green: u32,
    pub colorless: u32,
}

impl ManaPool {
    #[must_use]
    pub const fn plus(self, symbol: cards::ManaSymbol) -> Self {
        match symbol {
            cards::ManaSymbol::White => Self {
                white: self.white.saturating_add(1),
                ..self
            },
            cards::ManaSymbol::Blue => Self {
                blue: self.blue.saturating_add(1),
                ..self
            },
            cards::ManaSymbol::Black => Self {
                black: self.black.saturating_add(1),
                ..self
            },
            cards::ManaSymbol::Red => Self {
                red: self.red.saturating_add(1),
                ..self
            },
            cards::ManaSymbol::Green => Self {
                green: self.green.saturating_add(1),
                ..self
            },
            cards::ManaSymbol::Generic(amount) => Self {
                colorless: self.colorless.saturating_add(amount),
                ..self
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::mana;

    #[test]
    fn test_usage() {
        let pool = mana![2, G];
        assert_eq!(pool.green, 1);
        assert_eq!(pool.colorless, 2);
        assert_eq!(
            cards::ManaPool::default()
                .plus(cards::ManaSymbol::Blue)
                .blue,
            1
        );
    }
}
