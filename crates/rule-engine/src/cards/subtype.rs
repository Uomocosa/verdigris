use crate::cards;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Subtype {
    Land(cards::LandType),
    Enchantment(cards::EnchantmentType),
    Creature(cards::CreatureType),
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

impl From<cards::CreatureType> for Subtype {
    fn from(creature_type: cards::CreatureType) -> Self {
        Self::Creature(creature_type)
    }
}
