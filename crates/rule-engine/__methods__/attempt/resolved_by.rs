use bevy::ecs::schedule::{IntoScheduleConfigs, ScheduleConfigs};
use bevy::ecs::system::ScheduleSystem;

use crate::comprehensive_rules;
use crate::rules::traits::event::Event;

pub fn resolved_by<E: Event, Marker>(
    default: impl IntoScheduleConfigs<ScheduleSystem, Marker>,
) -> ScheduleConfigs<ScheduleSystem> {
    (
        comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
            E,
        >,
        default,
    )
        .chain()
}

#[cfg(test)]
mod tests {
    use bevy::ecs::schedule::{Chain, ScheduleConfigs};

    use crate::comprehensive_rules;
    use crate::events;

    use super::resolved_by;

    #[test]
    fn test_usage() {
        let configs =
            resolved_by::<events::Untap, _>(comprehensive_rules::r701_26b_untap::r701_26b_untap);
        let ScheduleConfigs::Configs {
            configs, metadata, ..
        } = configs
        else {
            panic!("resolved_by returns a group of systems");
        };
        assert_eq!(configs.len(), 2);
        assert!(matches!(metadata, Chain::Chained(_)));
    }
}
