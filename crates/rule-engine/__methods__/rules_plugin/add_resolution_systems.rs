use bevy::prelude::{App, IntoScheduleConfigs, Update};

use crate::RuleSet;
use crate::comprehensive_rules;
use crate::rules;

pub fn add_resolution_systems(app: &mut App) {
    app.add_systems(
        Update,
        (
            (
                comprehensive_rules::r117_4_priority::r117_4_priority,
                comprehensive_rules::r608_2a_intervening_if::r608_2a_intervening_if,
                comprehensive_rules::r608_2b_illegal_targets::r608_2b_illegal_targets,
                comprehensive_rules::r608_2_r608_2c_resolve::r608_2_r608_2c_resolve,
                comprehensive_rules::r611_2a_continuous_effects::r611_2a_continuous_effects,
            )
                .chain(),
            (
                (
                    comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                        rules::Draw,
                    >,
                    comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                        rules::Discard,
                    >,
                    comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                        rules::Mill,
                    >,
                    comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                        rules::CreateTokens,
                    >,
                    comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                        rules::Counter,
                    >,
                    comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                        rules::DealDamage,
                    >,
                )
                    .chain(),
                (
                    (
                        comprehensive_rules::r120_3_r120_3a_r120_3b_r120_3e_damage_results::r120_3_r120_3a_r120_3b_r120_3e_damage_results,
                        comprehensive_rules::r702_15b_lifelink::r702_15b_lifelink,
                    )
                        .chain(),
                    (
                        comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                            rules::LoseLife,
                        >,
                        comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                            rules::GainLife,
                        >,
                    )
                        .chain(),
                )
                    .chain(),
            )
                .chain(),
            (
                comprehensive_rules::r121_1_r121_2_draw::r121_1_r121_2_draw,
                comprehensive_rules::r701_17a_r701_17b_mill::r701_17a_r701_17b_mill,
                comprehensive_rules::r119_3_life_loss::r119_3_life_loss,
                comprehensive_rules::r701_9a_r701_9b_discard::r701_9a_r701_9b_discard,
                comprehensive_rules::r701_6a_counter::r701_6a_counter,
                comprehensive_rules::r510_3a_combat_damage_triggers::r510_3a_combat_damage_triggers,
                comprehensive_rules::r608_2n_graveyard::r608_2n_graveyard,
                comprehensive_rules::r608_3a_permanent_enters::r608_3a_permanent_enters,
                comprehensive_rules::r701_7a_create_tokens::r701_7a_create_tokens,
                comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                    rules::BeginTheGameOnTheBattlefield,
                >,
                comprehensive_rules::r103_6a_begin_the_game_on_the_battlefield::r103_6a_begin_the_game_on_the_battlefield,
                comprehensive_rules::r614_1a_r616_1_replacement_effects::r614_1a_r616_1_replacement_effects::<
                    rules::EnterBattlefield,
                >,
                comprehensive_rules::r110_1_enter_battlefield::r110_1_enter_battlefield,
                comprehensive_rules::r110_5b_enters_untapped::r110_5b_enters_untapped,
                (
                    comprehensive_rules::r608_3e_cannot_enter::r608_3e_cannot_enter,
                )
                    .chain(),
            )
                .chain(),
        )
            .chain()
            .in_set(RuleSet::Resolution),
    );
}

#[cfg(test)]
mod tests {
    use bevy::prelude::{App, Schedule, Update};

    use super::add_resolution_systems;

    #[test]
    fn test_usage() {
        let mut app = App::new();
        add_resolution_systems(&mut app);

        assert_eq!(
            app.get_schedule(Update).map(Schedule::systems_len),
            Some(30)
        );
    }
}
