use crate::cards;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Subtype {
    Land(cards::LandType),
    Enchantment(cards::EnchantmentType),
}

impl From<cards::LandType> for Subtype {
    fn from(land_type: cards::LandType) -> Self {
        Self::Land(land_type)
    }
}

impl From<cards::EnchantmentType> for Subtype {
    fn from(enchantment_type: cards::EnchantmentType) -> Self {
        Self::Enchantment(enchantment_type)
    }
}
