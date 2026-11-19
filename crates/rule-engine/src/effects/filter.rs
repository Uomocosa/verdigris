use atomic_delegate_macros::atomic_delegates;

use crate::effects;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Filter {
    pub kind: effects::FilterKind,
    pub controlled_by: Option<effects::PlayerRef>,
}

#[atomic_delegates]
impl Filter {
    #[must_use]
    pub fn controlled_by(self, player: effects::PlayerRef) -> Self {}
}

#[cfg(test)]
mod tests {
    use crate::ANY_TARGET;
    use crate::CREATURE;
    use crate::effects;

    #[test]
    fn test_usage() {
        assert_eq!(CREATURE.kind, effects::FilterKind::Creature);
        assert_eq!(ANY_TARGET.kind, effects::FilterKind::AnyTarget);
        assert_eq!(
            CREATURE
                .controlled_by(effects::PlayerRef::You)
                .controlled_by,
            Some(effects::PlayerRef::You)
        );
    }
}
