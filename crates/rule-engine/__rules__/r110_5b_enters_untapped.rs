use bevy::prelude::{Commands, MessageReader, Query};

use crate::cards;

/// 110.5b Permanents enter the battlefield untapped, unflipped, face up, and phased in unless a
/// spell or ability says otherwise.
pub fn r110_5b_enters_untapped(
    mut entered: MessageReader<cards::Entered>,
    mut commands: Commands,
    definitions: Query<&cards::CardDefinition>,
) {
    for entry in entered.read() {
        let Ok(definition) = definitions.get(**entry) else {
            continue;
        };
        if definition.enters_tapped() {
            commands.entity(**entry).insert(cards::Tapped);
        } else {
            commands.entity(**entry).remove::<cards::Tapped>();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::testing::*;
    use crate::{BATTLEFIELD, HAND};

    const GRUUL_GUILDGATE: Card = Card {
        name: TestCardName("Gruul Guildgate"),
        definition: || cards::land("Gruul Guildgate", []).with_enters_tapped(),
    };

    #[test]
    fn a_guildgate_that_enters_tapped_is_tapped_when_it_hits_the_battlefield() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([GRUUL_GUILDGATE]), P2]);
        let [guildgate] = game.player(p1).cards_in(HAND).exactly::<1>(GRUUL_GUILDGATE)?;

        game.play([p1.play_land(guildgate)])?;

        assert!(game.player(p1).cards_in(BATTLEFIELD).has(guildgate));
        assert!(game.is_tapped(guildgate));
        Ok(())
    }

    #[test]
    fn a_permanent_without_an_enters_tapped_clause_enters_untapped() -> TestResult {
        let (mut game, [p1, ..]) = MockGame::new([P1.hand([FOREST]), P2]);
        let [forest] = game.player(p1).cards_in(HAND).exactly::<1>(FOREST)?;

        game.play([p1.play_land(forest)])?;

        assert!(game.player(p1).cards_in(BATTLEFIELD).has(forest));
        assert!(!game.is_tapped(forest));
        Ok(())
    }
}
