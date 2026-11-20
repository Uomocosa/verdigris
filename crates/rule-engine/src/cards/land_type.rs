use crate::cards;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LandType {
    Plains,
    Island,
    Swamp,
    Mountain,
    Forest,
}

impl LandType {
    #[must_use]
    pub const fn mana_symbol(self) -> cards::ManaSymbol {
        match self {
            Self::Plains => cards::ManaSymbol::White,
            Self::Island => cards::ManaSymbol::Blue,
            Self::Swamp => cards::ManaSymbol::Black,
            Self::Mountain => cards::ManaSymbol::Red,
            Self::Forest => cards::ManaSymbol::Green,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;

    use super::LandType;

    #[test]
    fn test_usage() {
        let symbols = [
            (LandType::Plains, cards::ManaSymbol::White),
            (LandType::Island, cards::ManaSymbol::Blue),
            (LandType::Swamp, cards::ManaSymbol::Black),
            (LandType::Mountain, cards::ManaSymbol::Red),
            (LandType::Forest, cards::ManaSymbol::Green),
        ];

        for (land_type, symbol) in symbols {
            assert_eq!(land_type.mana_symbol(), symbol);
        }
    }
}
