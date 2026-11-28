use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;
use verdigris_datalog_ir::Facts;
use verdigris_datalog_ir::Program;
use verdigris_datalog_ir::Rel;
use verdigris_rule_engine::players;
use verdigris_rule_engine::rules;

use crate::TranslationError;

#[derive(Debug, Clone, Copy)]
pub struct PlayDatalog;

#[atomic_delegates]
impl PlayDatalog {
    #[must_use]
    pub fn snapshot(answers: &rules::Rules, viewer: players::PlayerId) -> Facts {}
    #[must_use]
    pub fn play_program() -> Result<Program, TranslationError> {}
    pub fn legality<G: verdigris_datalog_ir::traits::engine::Engine>(
        engine: &G,
        answers: &rules::Rules,
        card: Entity,
        player: players::PlayerId,
    ) -> Result<Option<rules::Legality>, TranslationError> {
    }
}

#[atomic_delegates]
impl PlayDatalog {
    #[must_use]
    pub(crate) fn visible_entities(
        answers: &rules::Rules,
        viewer: players::PlayerId,
    ) -> Vec<Entity> {
    }
    #[must_use]
    pub(crate) fn holder_changes(
        answers: &rules::Rules,
        event: &verdigris_rule_engine::events::Play,
        target: rules::Target,
    ) -> Vec<rules::Change> {
    }
    pub(crate) fn entity_facts(
        answers: &rules::Rules,
        facts: &mut Facts,
        entity: bevy::prelude::Entity,
    ) {
    }
    pub(crate) fn pair_facts(
        answers: &rules::Rules,
        facts: &mut Facts,
        card: bevy::prelude::Entity,
        player: players::PlayerId,
    ) {
    }
    #[must_use]
    pub(crate) fn waived(
        answers: &rules::Rules,
        event: &verdigris_rule_engine::events::Play,
        id: rules::RestrictionId,
        unwaivable: bool,
    ) -> bool {
    }
    pub(crate) fn translate_condition(
        condition: &rules::Condition,
        card: &rules::ArgRef,
        holder: &rules::ArgRef,
    ) -> Result<Vec<verdigris_datalog_ir::Literal>, TranslationError> {
    }
    pub(crate) fn translate_holds(
        condition: &rules::Condition,
        card: &rules::ArgRef,
        holder: &rules::ArgRef,
        candidate: &verdigris_datalog_ir::Atom,
        holds: &mut Vec<verdigris_datalog_ir::Rule>,
        next_hold: &mut usize,
    ) -> Result<verdigris_datalog_ir::Atom, TranslationError> {
    }
    pub(crate) fn holds_head(index: usize) -> Result<verdigris_datalog_ir::Atom, TranslationError> {
    }
}

impl PlayDatalog {
    pub(crate) const PLAY: &'static str = "play";
    pub(crate) const CARD_VAR: &'static str = "C";
    pub(crate) const PLAYER_VAR: &'static str = "P";
    pub(crate) const OBJECT: Rel = Rel("object");
    pub(crate) const PLAYER_FACT: Rel = Rel("player");
    pub(crate) const PLAY_CANDIDATE: Rel = Rel("play_candidate");
    pub(crate) const CARD_TYPE: Rel = Rel("card_type");
    pub(crate) const IN_ZONE: Rel = Rel("in_zone");
    pub(crate) const OWNER: Rel = Rel("owner");
    pub(crate) const PRIORITY: Rel = Rel("priority");
    pub(crate) const ACTIVE: Rel = Rel("active");
    pub(crate) const STEP: Rel = Rel("step");
    pub(crate) const MAIN_STEP: Rel = Rel("main_step");
    pub(crate) const ON_STACK: Rel = Rel("on_stack");
    pub(crate) const STACK_EMPTY: Rel = Rel("stack_empty");
    pub(crate) const LAND_PLAY_AVAILABLE: Rel = Rel("land_play_available");
    pub(crate) const WAIVED: Rel = Rel("waived");
    pub(crate) const VIOLATES: Rel = Rel("violates");
    pub(crate) const ADDED: Rel = Rel("added");
    pub(crate) const LAND: &'static str = "Land";
    pub(crate) const HAND_ZONE: &'static str = "Hand";
    pub(crate) const HOLDS_NAMES: [&str; 64] = [
        "holds_0", "holds_1", "holds_2", "holds_3", "holds_4", "holds_5", "holds_6", "holds_7",
        "holds_8", "holds_9", "holds_10", "holds_11", "holds_12", "holds_13", "holds_14",
        "holds_15", "holds_16", "holds_17", "holds_18", "holds_19", "holds_20", "holds_21",
        "holds_22", "holds_23", "holds_24", "holds_25", "holds_26", "holds_27", "holds_28",
        "holds_29", "holds_30", "holds_31", "holds_32", "holds_33", "holds_34", "holds_35",
        "holds_36", "holds_37", "holds_38", "holds_39", "holds_40", "holds_41", "holds_42",
        "holds_43", "holds_44", "holds_45", "holds_46", "holds_47", "holds_48", "holds_49",
        "holds_50", "holds_51", "holds_52", "holds_53", "holds_54", "holds_55", "holds_56",
        "holds_57", "holds_58", "holds_59", "holds_60", "holds_61", "holds_62", "holds_63",
    ];
}

#[cfg(test)]
mod tests {
    use verdigris_datalog_ir::Rel;

    use super::PlayDatalog;

    #[test]
    fn test_usage() {
        assert_eq!(*PlayDatalog::VIOLATES, "violates");
        assert_eq!(*PlayDatalog::PLAY_CANDIDATE, "play_candidate");
        assert_eq!(PlayDatalog::PLAY, "play");
        assert_eq!(Rel(PlayDatalog::PLAY), Rel("play"));
    }
}
