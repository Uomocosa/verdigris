use crate::effects;
use crate::rules;
use crate::rules::traits::event::Event;

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
    pub fn keyword_modifications(self) -> Vec<effects::Modification> {
        match self {
            Self::Haste => vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Clause(rules::declare_attacker::SUMMONING_SICK.key),
                change: rules::Change::AsThough,
            })],
            Self::Vigilance => vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Event(rules::TapAttacker::ID),
                change: rules::Change::Skip,
            })],
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::effects;
    use crate::keywords;
    use crate::rules;
    use crate::rules::traits::event::Event;

    use super::StaticKeyword;

    #[test]
    fn test_usage() {
        assert_eq!(StaticKeyword::Reach.keyword_modifications(), vec![]);
        assert_eq!(
            StaticKeyword::Haste.keyword_modifications(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Clause(rules::declare_attacker::SUMMONING_SICK.key),
                change: rules::Change::AsThough,
            })]
        );
        assert_eq!(
            keywords::StaticKeyword::Vigilance.keyword_modifications(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Event(rules::TapAttacker::ID),
                change: rules::Change::Skip,
            })]
        );
    }
}
