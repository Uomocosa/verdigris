use crate::STACK;
use bevy::prelude::{Entity, MessageWriter, Query, Res, ResMut};

use crate::cards;
use crate::rules::helpers;
use crate::stack;
use crate::turns;
use crate::zones;

/// 117.4 If all players pass in succession (that is, if all players pass without taking any
/// actions in between passing), the spell or ability on top of the stack resolves or, if the
/// stack is empty, the phase or step ends.
pub fn r117_4_priority(
    mut resolutions: MessageWriter<stack::Resolution>,
    mut all_players_passed: MessageWriter<turns::AllPlayersPassed>,
    mut priority: ResMut<turns::Priority>,
    turn: Res<turns::Turn>,
    turn_order: Res<turns::TurnOrder>,
    cards: Query<(&zones::InZone, Option<&stack::CastingStage>)>,
    zone_entities: Query<(Entity, &zones::Zone, Option<&cards::Owner>)>,
    stack_contents: Query<&zones::ZoneContents>,
) {
    let turn = turn.into_inner();
    let turn_order = turn_order.into_inner();
    let player_count = u32::try_from(turn_order.len()).unwrap_or(u32::MAX);
    if priority.consecutive_passes < player_count {
        return;
    }
    priority.consecutive_passes = 0;
    let top = helpers::find_zone(STACK, None, &zone_entities)
        .and_then(|stack| stack_contents.get(stack).ok())
        .and_then(|contents| contents.last().copied());
    let Some(top) = top else {
        all_players_passed.write(turns::AllPlayersPassed);
        return;
    };
    let is_still_casting = cards.get(top).is_ok_and(|(_, stage)| stage.is_some());
    if is_still_casting {
        return;
    }
    priority.player = turn.active;
    resolutions.write(stack::Resolution(top));
}

#[cfg(test)]
mod tests {
    use crate::mana;
    use crate::testing::*;
    use crate::turns;
    use crate::{GRAVEYARD, HAND, STACK};

    #[test]
    fn test_usage() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let [bolt] =
            game.player(p1).cards_in(HAND).matching(LIGHTNING_BOLT).exactly::<1>().unwrap();
        game.play([
            p1.cast(bolt).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();
        assert_eq!(game.zone_of(bolt), GRAVEYARD);
    }

    #[test]
    fn all_players_passing_resolves_one_spell() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, LIGHTNING_BOLT]).mana_pool(mana![R, R]),
            P2,
        ]);
        let [first, second] =
            game.player(p1).cards_in(HAND).matching(LIGHTNING_BOLT).exactly::<2>().unwrap();
        game.play([
            p1.cast(first).targeting(p2),
            p1.cast(second).targeting(p2).pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();

        assert_eq!(game.zone_of(second), GRAVEYARD);
        assert_eq!(game.zone_of(first), STACK);
        assert_eq!(game.life(p2), 17);
    }

    #[test]
    fn two_rounds_of_passing_resolves_both_spells() {
        let (mut game, [p1, p2]) = MockGame::new([
            P1.hand([LIGHTNING_BOLT, LIGHTNING_BOLT]).mana_pool(mana![R, R]),
            P2,
        ]);
        let [first, second] =
            game.player(p1).cards_in(HAND).matching(LIGHTNING_BOLT).exactly::<2>().unwrap();
        game.play([
            p1.cast(first).targeting(p2),
            p1.cast(second).targeting(p2).pass_priority(),
            p2.pass_priority(),
            p1.pass_priority(),
            p2.pass_priority(),
        ])
        .unwrap();

        assert_eq!(game.zone_of(second), GRAVEYARD);
        assert_eq!(game.zone_of(first), GRAVEYARD);
        assert_eq!(game.life(p2), 14);
    }

    #[test]
    fn priority_returns_to_the_active_player_after_a_spell_resolves() {
        let (mut game, [p1, p2]) =
            MockGame::new([P1.hand([LIGHTNING_BOLT]).mana_pool(mana![R]), P2]);
        let [bolt] =
            game.player(p1).cards_in(HAND).matching(LIGHTNING_BOLT).exactly::<1>().unwrap();
        game.play([p1.cast(bolt).targeting(p2).pass_priority()]).unwrap();

        assert_ne!(game.app.world().resource::<turns::Priority>().player, p1);
        game.play([p2.pass_priority()]).unwrap();

        let priority = *game.app.world().resource::<turns::Priority>();
        assert_eq!(priority.player, p1);
        assert_eq!(priority.consecutive_passes, 0);
    }
}
