use atomic_delegate_macros::atomic_delegate;
use bevy::prelude::Entity;
use verdigris_event_derive::EventArgs;

use crate::players;

use crate::rules;
use crate::rules::traits::event::Event;

pub const CREATURE: rules::RestrictionId = rules::RestrictionId("508.1a/creature");
pub const CONTROLS: rules::RestrictionId = rules::RestrictionId("508.1a/controls");
pub const UNTAPPED: rules::RestrictionId = rules::RestrictionId("508.1a/untapped");
pub const SUMMONING_SICK: rules::RestrictionId =
    rules::RestrictionId("508.1a/controlled-since-turn-began");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EventArgs)]
pub struct DeclareAttacker {
    #[arg]
    pub attacker: Entity,
    #[arg]
    pub player: players::PlayerId,
}

impl Event for DeclareAttacker {
    const ID: rules::EventId = rules::EventId("508.1a/declare-attacker");

    #[atomic_delegate(DeclareAttacker, Event)]
    fn restrictions() -> Vec<rules::Restriction> {}
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::effects;
    use crate::events;
    use crate::keywords;
    use crate::rules;
    use crate::rules::traits::event::Event;
    use crate::rules::traits::event_args::EventArgs;

    use super::CONTROLS;
    use super::CREATURE;
    use super::DeclareAttacker;
    use super::SUMMONING_SICK;
    use super::UNTAPPED;

    #[test]
    fn test_usage() {
        assert_eq!(
            DeclareAttacker::ID,
            rules::EventId("508.1a/declare-attacker")
        );
        let restrictions = DeclareAttacker::restrictions();
        assert_eq!(restrictions.len(), 4);
        assert_eq!(restrictions[0].id, CREATURE);
        assert_eq!(
            restrictions[0].text,
            "The active player chooses which creatures that they control"
        );
        assert_eq!(
            restrictions[0].condition,
            rules::has::<cards::Creature>().of(DeclareAttacker::ATTACKER)
        );
        assert_eq!(restrictions[1].id, CONTROLS);
        assert_eq!(restrictions[1].text, "creatures that they control");
        assert_eq!(
            restrictions[1].condition,
            (rules::points_to::<cards::Controller>(DeclareAttacker::PLAYER)
                & rules::has::<cards::OnBattlefield>())
            .of(DeclareAttacker::ATTACKER)
        );
        assert_eq!(restrictions[2].id, UNTAPPED);
        assert_eq!(
            restrictions[2].text,
            "The chosen creatures must be untapped"
        );
        assert_eq!(
            restrictions[2].condition,
            (!rules::has::<cards::Tapped>()).of(DeclareAttacker::ATTACKER)
        );
        assert_eq!(restrictions[3].id, SUMMONING_SICK);
        assert_eq!(
            restrictions[3].condition,
            (!rules::has::<cards::SummoningSick>()).of(DeclareAttacker::ATTACKER)
        );
        assert_eq!(
            DeclareAttacker::args(),
            &[DeclareAttacker::ATTACKER, DeclareAttacker::PLAYER,]
        );
    }

    #[test]
    fn restriction_texts_occur_verbatim_in_comprehensive_rules() {
        const RULES_TEXT: &str = include_str!("../../../../docs/MagicCompRules-20250919.txt");
        let keyword_restrictions = [
            keywords::Deathtouch::abilities(),
            keywords::Defender::abilities(),
            keywords::DoubleStrike::abilities(),
            keywords::FirstStrike::abilities(),
            keywords::Flying::abilities(),
            keywords::Haste::abilities(),
            keywords::Hexproof::abilities(),
            keywords::Indestructible::abilities(),
            keywords::Infect::abilities(),
            keywords::Lifelink::abilities(),
            keywords::Reach::abilities(),
            keywords::Trample::abilities(),
            keywords::Vigilance::abilities(),
        ]
        .into_iter()
        .flatten()
        .filter_map(|modification| match modification {
            effects::Modification::Rule(change) => match change.change {
                rules::Change::Restrict { restriction, .. } => Some(restriction),
                _ => None,
            },
            effects::Modification::Characteristic(_) => None,
        });
        for restriction in events::DeclareBlocker::restrictions()
            .into_iter()
            .chain(DeclareAttacker::restrictions())
            .chain(keyword_restrictions)
        {
            assert!(
                RULES_TEXT.contains(restriction.text),
                "restriction {:?} text not found verbatim in Comprehensive Rules: {:?}",
                restriction.id,
                restriction.text
            );
        }
    }
}
