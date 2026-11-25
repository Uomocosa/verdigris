use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::RuleSet;
use crate::actions;
use crate::comprehensive_rules;
use crate::events;
use crate::rules;
use crate::turns;

pub fn add_turn_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            comprehensive_rules::r103_6_opening_hand_actions::r103_6_opening_hand_actions(),
            comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                events::OpeningHandAction,
            >,
            rules::bevy_systems::show_may::show_may::<events::OpeningHandAction>,
            actions::bevy_systems::answer_asks::answer_asks,
            rules::bevy_systems::answer_may::answer_may::<events::OpeningHandAction>,
            turns::bevy_systems::opening_hand_actions_complete::opening_hand_actions_complete,
            comprehensive_rules::r103_8_first_turn::r103_8_first_turn,
            comprehensive_rules::r117_3a_r117_3c_r117_3d_priority::r117_3a_r117_3c_r117_3d_priority,
        )
            .chain()
            .in_set(RuleSet::Turn),
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Schedule, Update};

    use super::add_turn_systems;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        add_turn_systems(&mut app);

        assert_eq!(app.get_schedule(Update).map(Schedule::systems_len), Some(8));
    }
}
