use bevy::prelude::Resource;
use derive_more::{Deref, From};

use crate::players;

#[derive(Resource, Debug, Clone, PartialEq, Eq, Deref, From)]
pub struct TurnOrder(Vec<players::PlayerId>);

impl TurnOrder {
    #[must_use]
    pub fn next_after(&self, player: players::PlayerId) -> Option<players::PlayerId> {
        let position = self.iter().position(|candidate| *candidate == player)?;
        self.iter().cycle().nth(position.saturating_add(1)).copied()
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

        assert_eq!(order.next_after(first), Some(second));
        assert_eq!(order.next_after(fourth), Some(first));
        assert_eq!(order.next_after(outsider), None);
    }
}
