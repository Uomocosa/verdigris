use bevy::ecs::query::{Has, QueryData};

use crate::players;

#[derive(QueryData)]
#[query_data(derive(Clone, Copy))]
pub struct Status {
    pub has_left_the_game: Has<players::LeftTheGame>,
    pub has_won: Has<players::Won>,
    pub has_lost: Has<players::Lost>,
    pub is_game_a_draw: Has<players::GameIsADraw>,
}
