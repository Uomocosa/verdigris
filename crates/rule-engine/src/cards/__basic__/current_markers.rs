use bevy::ecs::query::{Has, QueryData};

use crate::cards;

#[derive(QueryData)]
#[query_data(derive(Clone, Copy))]
pub struct CurrentMarkers<'a> {
    pub on_battlefield: Has<cards::OnBattlefield>,
    pub effective: Option<&'a cards::EffectivePT>,
    pub indestructible: Has<cards::Indestructible>,
    pub deathtouch: Has<cards::Deathtouch>,
    pub infect: Has<cards::Infect>,
    pub lifelink: Has<cards::Lifelink>,
    pub trample: Has<cards::Trample>,
    pub flying: Has<cards::Flying>,
    pub reach: Has<cards::Reach>,
    pub vigilance: Has<cards::Vigilance>,
    pub enters: Option<&'a cards::TriggeredEnters>,
    pub at_step: Option<&'a cards::TriggeredAtStep>,
}
