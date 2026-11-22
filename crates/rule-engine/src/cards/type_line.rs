use atomic_delegate_macros::atomic_delegates;

use crate::cards;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TypeLine {
    pub supertypes: Vec<cards::Supertype>,
    pub types: Vec<cards::CardType>,
    pub subtypes: Vec<cards::Subtype>,
}

#[atomic_delegates]
impl TypeLine {
    #[must_use]
    pub fn land_types(&self) -> impl Iterator<Item = cards::LandType> + '_ {}
}

#[cfg(test)]
mod tests {
    use crate::cards;

    use super::TypeLine;

    #[test]
    fn test_usage() {
        let type_line = TypeLine {
            supertypes: vec![cards::Supertype::Basic],
            types: vec![cards::CardType::Land],
            subtypes: vec![cards::Subtype::from(cards::LandType::Forest)],
        };
        assert_eq!(type_line.supertypes, vec![cards::Supertype::Basic]);
        assert_eq!(type_line.types, vec![cards::CardType::Land]);
    }
}
