use crate::cards;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ManaType {
    Color(cards::Color),
    Colorless,
}

impl From<cards::Color> for ManaType {
    fn from(color: cards::Color) -> Self {
        Self::Color(color)
    }
}

impl ManaType {
    #[must_use]
    pub const fn mana_symbol(self) -> cards::ManaSymbol {
        match self {
            Self::Color(cards::Color::White) => cards::ManaSymbol::White,
            Self::Color(cards::Color::Blue) => cards::ManaSymbol::Blue,
            Self::Color(cards::Color::Black) => cards::ManaSymbol::Black,
            Self::Color(cards::Color::Red) => cards::ManaSymbol::Red,
            Self::Color(cards::Color::Green) => cards::ManaSymbol::Green,
            Self::Colorless => cards::ManaSymbol::Colorless,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;

    use super::ManaType;

    #[test]
    fn test_usage() {
        let green = ManaType::from(cards::Color::Green);
        assert_eq!(green, ManaType::Color(cards::Color::Green));
        assert_eq!(green.mana_symbol(), cards::ManaSymbol::Green);
        assert_eq!(
            ManaType::Colorless.mana_symbol(),
            cards::ManaSymbol::Colorless
        );
    }
}
