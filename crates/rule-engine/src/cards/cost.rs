use crate::cards;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Cost {
    pub white: u8,
    pub blue: u8,
    pub black: u8,
    pub red: u8,
    pub green: u8,
    pub generic: u32,
    pub variable: u32,
}

impl Cost {
    #[must_use]
    pub const fn with_variable(mut self) -> Self {
        self.variable = self.variable.saturating_add(1);
        self
    }

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
                generic: self.generic.saturating_add(amount),
                ..self
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;

    #[test]
    fn test_usage() {
        let cost = cost![1, B, B];
        assert_eq!(cost.generic, 1);
        assert_eq!(cost.black, 2);
        assert_eq!(cost.red, 0);
        assert_eq!(cards::Cost::default().plus(cards::ManaSymbol::Red).red, 1);
        assert_eq!(cost.variable, 0);
        assert_eq!(cards::Cost::default().with_variable().variable, 1);
    }
}
