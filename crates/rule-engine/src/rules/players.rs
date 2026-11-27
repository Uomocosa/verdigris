use crate::players;
use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Players {
    You,
    Opponents,
    All,
}

#[rustfmt::skip]
impl From<rules::Each> for Players {
    fn from(each: rules::Each) -> Self { *each }
}

impl Players {
    #[must_use]
    pub fn resolve(
        self,
        you: Option<players::PlayerId>,
        in_game: &[players::PlayerId],
    ) -> Vec<players::PlayerId> {
        match (self, you) {
            (Self::You, Some(player)) => vec![player],
            (Self::Opponents, Some(player)) => {
                in_game.iter().copied().filter(|candidate| *candidate != player).collect()
            }
            (Self::You | Self::Opponents, None) => Vec::new(),
            (Self::All, _) => in_game.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::players;

    use super::Players;

    #[test]
    fn test_usage() {
        let first = players::PlayerId(Entity::from_raw_u32(1).unwrap());
        let second = players::PlayerId(Entity::from_raw_u32(2).unwrap());
        let in_game = vec![first, second];
        assert_eq!(Players::You.resolve(Some(first), &in_game), vec![first]);
        assert_eq!(
            Players::Opponents.resolve(Some(first), &in_game),
            vec![second]
        );
        assert_eq!(Players::All.resolve(Some(first), &in_game), in_game);
        assert_eq!(Players::You.resolve(None, &in_game), Vec::new());
        assert_eq!(Players::Opponents.resolve(None, &in_game), Vec::new());
    }
}
