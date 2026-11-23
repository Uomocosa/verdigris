use crate::effects;
use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StaticKeyword {
    Enchant(effects::Filter),
    Indestructible,
    Deathtouch,
    Lifelink,
    Trample,
    Flying,
    Reach,
    Vigilance,
    Defender,
    Haste,
    Hexproof,
    FirstStrike,
    DoubleStrike,
    Infect,
}

impl StaticKeyword {
    #[must_use]
    pub fn keyword_changes(self) -> Vec<effects::Change> {
        match self {
            Self::Reach => vec![effects::Change::Waive(rules::FLYING.key)],
            Self::Haste => vec![effects::Change::Waive(rules::SUMMONING_SICK.key)],
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::keywords;
    use crate::rules;

    use super::StaticKeyword;

    #[test]
    fn test_usage() {
        assert_eq!(
            StaticKeyword::Reach.keyword_changes(),
            vec![effects::Change::Waive(rules::FLYING.key)]
        );
        assert_eq!(
            StaticKeyword::Haste.keyword_changes(),
            vec![effects::Change::Waive(rules::SUMMONING_SICK.key)]
        );
        assert_eq!(
            keywords::StaticKeyword::Vigilance.keyword_changes(),
            Vec::new()
        );
    }
}
