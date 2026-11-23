use crate::TAP_ATTACKERS;
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
            Self::Haste => vec![effects::Change::Waive(
                rules::declare_attacker::SUMMONING_SICK.key,
            )],
            Self::Vigilance => vec![effects::Change::Skip(TAP_ATTACKERS)],
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TAP_ATTACKERS;
    use crate::effects;
    use crate::keywords;
    use crate::rules;

    use super::StaticKeyword;

    #[test]
    fn test_usage() {
        assert_eq!(StaticKeyword::Reach.keyword_changes(), vec![]);
        assert_eq!(
            StaticKeyword::Haste.keyword_changes(),
            vec![effects::Change::Waive(
                rules::declare_attacker::SUMMONING_SICK.key
            )]
        );
        assert_eq!(
            keywords::StaticKeyword::Vigilance.keyword_changes(),
            vec![effects::Change::Skip(TAP_ATTACKERS)]
        );
    }
}
