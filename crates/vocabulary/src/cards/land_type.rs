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
    pub fn from_subtype(id: cards::SubtypeId) -> Option<Self> {
        match id {
            cards::PLAINS => Some(Self::Plains),
            cards::ISLAND => Some(Self::Island),
            cards::SWAMP => Some(Self::Swamp),
            cards::MOUNTAIN => Some(Self::Mountain),
            cards::FOREST => Some(Self::Forest),
            _ => None,
        }
    }

    #[must_use]
    pub const fn subtype(self) -> cards::SubtypeId {
        match self {
            Self::Plains => cards::PLAINS,
            Self::Island => cards::ISLAND,
            Self::Swamp => cards::SWAMP,
            Self::Mountain => cards::MOUNTAIN,
            Self::Forest => cards::FOREST,
        }
    }

    #[must_use]
    pub const fn color(self) -> cards::Color {
        match self {
            Self::Plains => cards::Color::White,
            Self::Island => cards::Color::Blue,
            Self::Swamp => cards::Color::Black,
            Self::Mountain => cards::Color::Red,
            Self::Forest => cards::Color::Green,
        }
    }

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

        let colors = [
            (LandType::Plains, cards::Color::White),
            (LandType::Island, cards::Color::Blue),
            (LandType::Swamp, cards::Color::Black),
            (LandType::Mountain, cards::Color::Red),
            (LandType::Forest, cards::Color::Green),
        ];

        for (land_type, color) in colors {
            assert_eq!(land_type.color(), color);
        }
    }
}
