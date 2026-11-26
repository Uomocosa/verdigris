use crate::cards;
use crate::effects;
use crate::events;
use crate::players;
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
    pub fn keyword_modifications(self) -> Vec<effects::Modification> {
        match self {
            Self::Haste => vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Clause(events::declare_attacker::SUMMONING_SICK.key),
                change: rules::Change::AsThough,
            })],
            Self::Vigilance => vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::TapAttacker>(),
                change: rules::Change::Skip,
            })],
            Self::Indestructible => vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::Destroy>(),
                change: rules::Change::Cant {
                    role: rules::Role::Subject,
                },
            })],
            Self::Hexproof => vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::BecomeTarget>(),
                change: rules::Change::OnlyIf {
                    role: rules::Role::Subject,
                    condition: rules::points_to::<cards::Controller>(rules::Role::Player)
                        | rules::is(rules::Role::Player),
                },
            })],
            Self::Trample => vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Clause(events::assign_combat_damage::ONLY_BLOCKERS.key),
                change: rules::Change::Allow(rules::has::<players::Player>()),
            })],
            Self::Deathtouch => vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::value::<rules::LethalDamage>(),
                change: rules::Change::Set(1),
            })],
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::effects;
    use crate::events;
    use crate::keywords;
    use crate::rules;

    use super::StaticKeyword;

    #[test]
    fn test_usage() {
        assert_eq!(StaticKeyword::Reach.keyword_modifications(), vec![]);
        assert_eq!(
            StaticKeyword::Haste.keyword_modifications(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::Clause(events::declare_attacker::SUMMONING_SICK.key),
                change: rules::Change::AsThough,
            })]
        );
        assert_eq!(
            keywords::StaticKeyword::Vigilance.keyword_modifications(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::TapAttacker>(),
                change: rules::Change::Skip,
            })]
        );
        assert_eq!(
            keywords::StaticKeyword::Hexproof.keyword_modifications(),
            vec![effects::Modification::Rule(rules::RuleChange {
                target: rules::Target::event::<events::BecomeTarget>(),
                change: rules::Change::OnlyIf {
                    role: rules::Role::Subject,
                    condition: rules::points_to::<cards::Controller>(rules::Role::Player)
                        | rules::is(rules::Role::Player),
                },
            })]
        );
    }
}
