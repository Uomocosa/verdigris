use bevy::prelude::Entity;

use crate::testing;
use crate::turns;

#[derive(Debug, Clone, Copy)]
pub struct ActiveAutoPass {
    pub(crate) player: Entity,
    pub(crate) until: testing::AutoPassUntil,
    pub(crate) start: turns::Turn,
}

impl ActiveAutoPass {
    #[must_use]
    pub(crate) fn is_stop_reached(&self, now: turns::Turn) -> bool {
        match self.until {
            testing::AutoPassUntil::StepStarts(target) => {
                now.step == target && (now.number, now.step) != (self.start.number, self.start.step)
            }
            testing::AutoPassUntil::NextStep => {
                (now.number, now.step) != (self.start.number, self.start.step)
            }
            testing::AutoPassUntil::NextPhase => {
                now.number != self.start.number || now.step.phase() != self.start.step.phase()
            }
            testing::AutoPassUntil::OwnNextTurn => {
                now.active == self.player && now.number > self.start.number
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Entity;

    use crate::testing;
    use crate::turns;

    use super::ActiveAutoPass;

    #[test]
    fn test_usage() {
        let start = turns::Turn {
            number: 1,
            active: Entity::PLACEHOLDER,
            step: turns::Step::PrecombatMain,
        };
        let auto_pass = ActiveAutoPass {
            player: Entity::PLACEHOLDER,
            until: testing::AutoPassUntil::NextStep,
            start,
        };

        assert!(!auto_pass.is_stop_reached(start));
        assert!(auto_pass.is_stop_reached(turns::Turn {
            number: 1,
            active: Entity::PLACEHOLDER,
            step: turns::Step::BeginningOfCombat,
        }));
    }
}
