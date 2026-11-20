use atomic_delegate_macros::atomic_delegates;
use bevy::prelude::Entity;

use crate::actions;
use crate::testing;
use crate::turns;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestPlayerActions {
    pub(crate) player: testing::TestPlayer,
    pub(crate) steps: Vec<testing::PlayStep>,
}

#[atomic_delegates]
impl TestPlayerActions {
    #[must_use]
    pub const fn of(player: testing::TestPlayer) -> Self {}

    #[must_use]
    pub fn from_cast(player: testing::TestPlayer, spell: Entity) -> Self {}
}

#[atomic_delegates]
impl TestPlayerActions {
    #[must_use]
    pub fn targeting(self, targets: impl Into<testing::Targets>) -> Self {}

    #[must_use]
    pub fn declare_attackers<const N: usize>(self, attackers: [testing::TestCard; N]) -> Self {}

    #[must_use]
    pub fn declare_blockers<const N: usize>(
        self,
        blocks: [(testing::TestCard, testing::TestCard); N],
    ) -> Self {
    }

    #[must_use]
    pub fn discard<const N: usize>(self, cards: [testing::TestCard; N]) -> Self {}
}

#[rustfmt::skip]
impl TestPlayerActions {
    #[must_use]
    pub fn pass_priority(self) -> Self { self.act(actions::Action::PassPriority) }

    #[must_use]
    pub fn play_land(self, land: testing::TestCard) -> Self { self.act(actions::Action::PlayLand(land.into())) }
}

impl TestPlayerActions {
    #[must_use]
    pub fn activate_ability(self, ability: testing::TestAbility) -> Self {
        let player = *self.player;
        self.with_step(testing::PlayStep::Activate(player, ability))
    }

    #[must_use]
    pub fn cast(self, spell: impl Into<Entity>) -> Self {
        let player = *self.player;
        let spell = spell.into();
        self.with_step(testing::PlayStep::Act(player, actions::Action::Cast(spell)))
    }

    #[must_use]
    pub fn pass_to(self, step: turns::Step) -> Self {
        let player = *self.player;
        self.with_step(testing::PlayStep::AutoPass {
            player,
            until: testing::AutoPassUntil::StepStarts(step),
        })
    }

    #[must_use]
    pub fn end_current_step(self) -> Self {
        let player = *self.player;
        self.with_step(testing::PlayStep::AutoPass {
            player,
            until: testing::AutoPassUntil::NextStep,
        })
    }

    #[must_use]
    pub fn end_phase(self) -> Self {
        let player = *self.player;
        self.with_step(testing::PlayStep::AutoPass {
            player,
            until: testing::AutoPassUntil::NextPhase,
        })
    }

    #[must_use]
    pub fn end_turn(self) -> Self {
        let player = *self.player;
        self.with_step(testing::PlayStep::AutoPass {
            player,
            until: testing::AutoPassUntil::OwnNextTurn,
        })
    }
}

impl TestPlayerActions {
    pub(crate) fn with_step(mut self, step: testing::PlayStep) -> Self {
        self.steps.push(step);
        self
    }

    pub(crate) fn act(self, action: actions::Action) -> Self {
        let player = *self.player;
        self.with_step(testing::PlayStep::Act(player, action))
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::World;

    use crate::actions;
    use crate::testing;
    use crate::turns;

    use super::TestPlayerActions;

    #[test]
    fn test_usage() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let spell = world.spawn_empty().id();
        let target = world.spawn_empty().id();

        let actions = testing::TestPlayer::from_entity_and_seat(caster, testing::PlayerSeat(1))
            .cast(spell)
            .targeting(target)
            .pass_to(turns::Step::PostcombatMain);

        assert_eq!(
            actions.steps,
            vec![
                testing::PlayStep::Act(caster, actions::Action::Cast(spell)),
                testing::PlayStep::Act(caster, actions::Action::ChooseTargets(vec![target])),
                testing::PlayStep::AutoPass {
                    player: caster,
                    until: testing::AutoPassUntil::StepStarts(turns::Step::PostcombatMain),
                },
            ]
        );

        assert_eq!(
            TestPlayerActions::of(testing::TestPlayer::from_entity_and_seat(
                caster,
                testing::PlayerSeat(1)
            ))
            .steps,
            Vec::new()
        );
    }

    #[test]
    fn declare_attackers_declare_blockers_play_land_and_discard_push_act_steps() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let attacker = world.spawn_empty().id();
        let blocker = world.spawn_empty().id();
        let land = world.spawn_empty().id();
        let card = world.spawn_empty().id();

        let actions = testing::TestPlayer::from_entity_and_seat(caster, testing::PlayerSeat(1))
            .declare_attackers([testing::TestCard(attacker)])
            .declare_blockers([(testing::TestCard(blocker), testing::TestCard(attacker))])
            .play_land(testing::TestCard(land))
            .discard([testing::TestCard(card)]);

        assert_eq!(
            actions.steps,
            vec![
                testing::PlayStep::Act(caster, actions::Action::DeclareAttackers(vec![attacker])),
                testing::PlayStep::Act(
                    caster,
                    actions::Action::DeclareBlockers(vec![actions::BlockAssignment {
                        blocker,
                        attacker,
                    }])
                ),
                testing::PlayStep::Act(caster, actions::Action::PlayLand(land)),
                testing::PlayStep::Act(caster, actions::Action::ChooseDiscard(vec![card])),
            ]
        );
    }

    #[test]
    fn activate_ability_pushes_an_activate_step() {
        let mut world = World::new();
        let caster = world.spawn_empty().id();
        let source = world.spawn_empty().id();
        let spell = world.spawn_empty().id();
        let ability = testing::TestAbility::Only { source };

        let first = testing::TestPlayer::from_entity_and_seat(caster, testing::PlayerSeat(1))
            .activate_ability(ability);

        assert_eq!(
            first.steps,
            vec![testing::PlayStep::Activate(caster, ability)]
        );

        let chained = testing::TestPlayer::from_entity_and_seat(caster, testing::PlayerSeat(1))
            .activate_ability(ability)
            .cast(spell);

        assert_eq!(
            chained.steps,
            vec![
                testing::PlayStep::Activate(caster, ability),
                testing::PlayStep::Act(caster, actions::Action::Cast(spell)),
            ]
        );
    }
}
