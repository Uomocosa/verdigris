use crate::players;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerRef {
    You,
    Opponent,
    Any,
}

impl PlayerRef {
    #[must_use]
    pub fn resolve(
        self,
        controller: players::PlayerId,
        players: &[players::PlayerId],
    ) -> Vec<players::PlayerId> {
        match self {
            Self::You => vec![controller],
            Self::Opponent => Self::players_other_than(players, controller),
            Self::Any => players.to_vec(),
        }
    }
}

#[rustfmt::skip]
impl PlayerRef {
    #[must_use]
    pub(crate) fn players_other_than(
        players: &[players::PlayerId],
        controller: players::PlayerId,
    ) -> Vec<players::PlayerId> { players.iter().copied().filter(|player| *player != controller).collect() }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::PlayerRef;

    #[test]
    fn test_usage() {
        let (_, [p1, p2, p3, p4]) = MockGame::new([P1, P2, P3, P4]);
        let players = vec![p1.id, p2.id, p3.id, p4.id];
        let controller = players[0];
        assert_eq!(
            PlayerRef::You.resolve(controller, &players),
            vec![controller]
        );
        assert_eq!(PlayerRef::Opponent.resolve(controller, &players).len(), 3);
        assert_eq!(PlayerRef::Any.resolve(controller, &players), players);
    }
}
