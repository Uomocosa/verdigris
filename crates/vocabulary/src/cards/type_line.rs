use crate::cards;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TypeLine {
    pub supertypes: Vec<cards::Supertype>,
    pub types: Vec<cards::CardType>,
    pub subtypes: Vec<cards::SubtypeId>,
}

#[rustfmt::skip]
impl TypeLine {
    pub fn land_types(&self) -> impl Iterator<Item = cards::LandType> + '_ { self.subtypes.iter().copied().filter_map(cards::LandType::from_subtype) }
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
            subtypes: vec![cards::FOREST],
        };
        assert_eq!(type_line.supertypes, vec![cards::Supertype::Basic]);
        assert_eq!(type_line.types, vec![cards::CardType::Land]);
    }

    #[test]
    fn land_types_returns_the_basic_land_types() {
        let type_line = TypeLine {
            subtypes: vec![cards::PLAINS, cards::ISLAND],
            ..TypeLine::default()
        };
        assert_eq!(
            type_line.land_types().collect::<Vec<_>>(),
            vec![cards::LandType::Plains, cards::LandType::Island]
        );
    }
}
