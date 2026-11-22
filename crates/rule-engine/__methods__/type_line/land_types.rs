use crate::cards;

pub fn land_types(type_line: &cards::TypeLine) -> impl Iterator<Item = cards::LandType> + '_ {
    type_line.subtypes.iter().filter_map(|subtype| match subtype {
        cards::Subtype::Land(land_type) => Some(*land_type),
        cards::Subtype::Enchantment(_) => None,
    })
}

#[cfg(test)]
mod tests {
    use crate::cards;

    use super::land_types;

    #[test]
    fn test_usage() {
        let type_line = cards::TypeLine {
            subtypes: vec![
                cards::Subtype::from(cards::LandType::Plains),
                cards::Subtype::from(cards::LandType::Island),
            ],
            ..cards::TypeLine::default()
        };
        assert_eq!(
            land_types(&type_line).collect::<Vec<_>>(),
            vec![cards::LandType::Plains, cards::LandType::Island]
        );
    }
}
