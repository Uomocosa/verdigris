use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::RuleSet;
use crate::rules;

pub fn add_layers_systems(app: &mut App) {
    app.add_systems(
        Update,
        rules::r613_1_layers::r613_1_layers.after(RuleSet::Turn).before(RuleSet::PriorityAction),
    );
    app.add_systems(
        Update,
        rules::r613_1_layers::r613_1_layers
            .after(RuleSet::PriorityAction)
            .before(RuleSet::Resolution),
    );
    app.add_systems(
        Update,
        rules::r613_1_layers::r613_1_layers.after(RuleSet::Resolution).before(RuleSet::StateBased),
    );
    app.add_systems(
        Update,
        rules::r613_1_layers::r613_1_layers.after(RuleSet::StateBased).before(RuleSet::StepEnd),
    );
    app.add_systems(
        Update,
        rules::r613_1_layers::r613_1_layers.after(RuleSet::StepEnd).before(RuleSet::StepStarted),
    );
    app.add_systems(
        Update,
        rules::r613_1_layers::r613_1_layers.after(RuleSet::StepStarted).before(RuleSet::Cleanup),
    );
    app.add_systems(
        Update,
        rules::r613_1_layers::r613_1_layers.after(RuleSet::Cleanup).before(RuleSet::TurnEnd),
    );
    app.add_systems(
        Update,
        rules::r613_1_layers::r613_1_layers.after(RuleSet::TurnEnd),
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Schedule, Update};

    use super::add_layers_systems;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        add_layers_systems(&mut app);

        assert_eq!(app.get_schedule(Update).map(Schedule::systems_len), Some(8));
    }
}
