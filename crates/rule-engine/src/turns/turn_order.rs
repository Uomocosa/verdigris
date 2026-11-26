use bevy::prelude::Resource;
use derive_more::{Deref, From};

use crate::players;

#[derive(Resource, Debug, Clone, PartialEq, Eq, Deref, From)]
pub struct TurnOrder(Vec<players::PlayerId>);

impl TurnOrder {
    pub fn starting_from(
        &self,
        first: players::PlayerId,
    ) -> impl Iterator<Item = players::PlayerId> + '_ {
        let start = self.iter().position(|player| *player == first).unwrap_or(0);
        self.iter().copied().cycle().skip(start).take(self.len())
    }
}

#[cfg(test)]
mod tests {
    use crate::testing::*;

    use super::TurnOrder;

    #[test]
    fn test_usage() {
        let (_, [p1, p2, p3, p4, p5]) = MockGame::new([P1, P2, P3, P4, P5]);
        let (first, second, third, fourth, outsider) = (p1.id, p2.id, p3.id, p4.id, p5.id);
        let order = TurnOrder::from(vec![first, second, third, fourth]);

        assert_eq!(
            order.starting_from(first).collect::<Vec<_>>(),
            [first, second, third, fourth]
        );
        assert_eq!(
            order.starting_from(fourth).collect::<Vec<_>>(),
            [fourth, first, second, third]
        );
        assert_eq!(
            order.starting_from(outsider).collect::<Vec<_>>(),
            [first, second, third, fourth]
        );
    }
}
