use bevy::ecs::query::{Has, QueryData};

use crate::cards;

#[derive(QueryData)]
#[query_data(derive(Clone, Copy))]
pub struct CurrentMarkers<'a> {
    pub on_battlefield: Has<cards::OnBattlefield>,
    pub effective: Option<&'a cards::EffectivePT>,
    pub enters: Option<&'a cards::TriggeredEnters>,
    pub at_step: Option<&'a cards::TriggeredAtStep>,
    pub creature: Has<cards::Creature>,
}
