use crate::cards;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Subtype {
    Land(cards::LandType),
}

impl From<cards::LandType> for Subtype {
    fn from(land_type: cards::LandType) -> Self {
        Self::Land(land_type)
    }
}
