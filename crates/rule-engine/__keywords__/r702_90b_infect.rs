use crate::LOSES_LIFE;
use crate::counters;
use crate::effects;
use crate::events;
use crate::keywords;
use crate::rules;

/// 702.90b Damage dealt to a player by a source with infect doesn’t cause that player to lose life.
/// Rather, it causes that source’s controller to give the player that many poison counters. See
/// rule 120.3.
///
/// # Restates
///
/// 120.3b Damage dealt to a player by a source with infect causes that source’s controller to give
/// the player that many poison counters.
#[must_use]
pub fn r702_90b_infect() -> keywords::Keyword {
    keywords::Keyword::from_plain(ID, abilities)
}

pub const ID: keywords::KeywordId = keywords::KeywordId("702.90/infect");

fn abilities() -> Vec<effects::Modification> {
    vec![effects::Modification::Rule(rules::RuleChange {
        target: rules::Target::Result(LOSES_LIFE),
        change: rules::Change::Instead {
            holder: events::DealDamage::SOURCE.into(),
            when: rules::Condition::True,
            then: rules::Then::Events(vec![rules::EventTemplate::PutCounters {
                who: events::DealDamage::RECIPIENT.into(),
                kind: counters::POISON,
                amount: rules::Amount::FromArg(events::DealDamage::AMOUNT),
            }]),
        },
    })]
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::counters;
    use crate::testing::*;
    use crate::turns;

    #[test]
    fn test_usage() {
        assert_eq!(super::r702_90b_infect().id, super::ID);
        assert_eq!(super::r702_90b_infect().abilities(), super::abilities());
    }

    #[test]
    fn infect_damage_gives_poison_counters_instead_of_losing_life() -> TestResult {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.battlefield([PLAGUE_STINGER]),
            P2.battlefield([GRIZZLY_BEARS]),
        ]);
        let [stinger] = game.player(p1).cards_in(BATTLEFIELD).exactly::<1>(PLAGUE_STINGER)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([stinger]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::CombatDamage),
            p2.pass_to(turns::Step::CombatDamage),
            p1.pass_priority(),
            p2.pass_priority(),
        ])?;

        assert_eq!(game.life(p2), 20);
        let mut expected = counters::Counters::default();
        expected.add(counters::POISON, 1);
        assert_eq!(game.get::<counters::Counters>(p2), Some(expected));
        Ok(())
    }
}
