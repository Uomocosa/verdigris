use crate::cards;

pub fn colors(definition: &cards::CardDefinition) -> Vec<cards::Color> {
    let mana_symbols_by_color = [
        (definition.cost.white, cards::Color::White),
        (definition.cost.blue, cards::Color::Blue),
        (definition.cost.black, cards::Color::Black),
        (definition.cost.red, cards::Color::Red),
        (definition.cost.green, cards::Color::Green),
    ];
    mana_symbols_by_color
        .into_iter()
        .filter(|(mana_symbols, _)| *mana_symbols > 0)
        .map(|(_, color)| color)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::pt;

    use super::colors;

    #[test]
    fn test_usage() {
        let multicolored = cards::creature("Gold Bear", cost![1, B, G], pt!(2, 2));
        assert_eq!(
            colors(&multicolored),
            vec![cards::Color::Black, cards::Color::Green]
        );
        let colorless = cards::creature("Artifact Bear", cards::Cost::default(), pt!(2, 2));
        assert_eq!(colors(&colorless), Vec::new());
    }
}
