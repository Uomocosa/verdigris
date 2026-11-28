use crate::DAMAGE_MARKED;
use crate::LOSES_LIFE;
use crate::MINUS_ONE_MINUS_ONE;
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
///
/// 120.3d Damage dealt to a creature by a source with wither and/or infect causes that source’s
/// controller to put that many -1/-1 counters on that creature.
#[must_use]
pub fn r702_90b_infect() -> keywords::Keyword {
    keywords::Keyword::from_plain(ID, abilities)
}

pub const ID: keywords::KeywordId = keywords::KeywordId("702.90/infect");

fn abilities() -> Vec<effects::Modification> {
    vec![
        effects::Modification::Rule(rules::RuleChange {
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
        }),
        effects::Modification::Rule(rules::RuleChange {
            target: rules::Target::Result(DAMAGE_MARKED),
            change: rules::Change::Instead {
                holder: events::DealDamage::SOURCE.into(),
                when: rules::Condition::True,
                then: rules::Then::Events(vec![rules::EventTemplate::PutCounters {
                    who: events::DealDamage::RECIPIENT.into(),
                    kind: MINUS_ONE_MINUS_ONE,
                    amount: rules::Amount::FromArg(events::DealDamage::AMOUNT),
                }]),
            },
        }),
    ]
}

#[cfg(test)]
mod tests {
    use crate::BATTLEFIELD;
    use crate::HAND;
    use crate::PLUS_ONE_PLUS_ONE;
    use crate::counters;
    use crate::damage;
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
            P1.battlefield([PLAGUE_STINGER, PLAGUE_STINGER]),
            P2.battlefield([GRIZZLY_BEARS, GIANT_SPIDER, FOREST]).hand([BATTLEGROWTH]),
        ]);
        let [stinger_1, stinger_2] =
            game.player(p1).cards_in(BATTLEFIELD).exactly::<2>(PLAGUE_STINGER)?;
        let [spider] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(GIANT_SPIDER)?;
        let [forest] = game.player(p2).cards_in(BATTLEFIELD).exactly::<1>(FOREST)?;
        let [battlegrowth] = game.player(p2).cards_in(HAND).exactly::<1>(BATTLEGROWTH)?;
        game.play([
            p1.pass_to(turns::Step::DeclareAttackers),
            p2.pass_to(turns::Step::DeclareAttackers),
            p1.declare_attackers([stinger_1, stinger_2]),
        ])?;
        game.play([
            p1.pass_to(turns::Step::DeclareBlockers),
            p2.pass_to(turns::Step::DeclareBlockers),
        ])?;
        game.play([
            p1.pass_priority(),
            p2.declare_blockers([(spider, stinger_1)]),
        ])?;
        game.play([
            p2.activate_ability(forest.ability())
                .cast(battlegrowth)
                .targeting(spider)
                .pass_priority(),
            p1.pass_priority(),
        ])?;
        let mut plus_one = counters::Counters::default();
        plus_one.add(PLUS_ONE_PLUS_ONE, 1);
        assert_eq!(game.get::<counters::Counters>(spider), Some(plus_one));
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
        assert_eq!(
            game.get::<counters::Counters>(spider),
            Some(counters::Counters::default())
        );
        assert_eq!(game.get::<damage::Damage>(spider), Some(damage::Damage(0)));
        Ok(())
    }
}
