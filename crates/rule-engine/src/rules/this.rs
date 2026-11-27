use crate::abilities;
use crate::rules::traits::dealt::Dealt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct This;

#[rustfmt::skip]
impl This {
    #[must_use]
    pub const fn dies(self) -> abilities::TriggerEvent { abilities::TriggerEvent::Dies }
    #[must_use]
    pub const fn attacks(self) -> abilities::TriggerEvent { abilities::TriggerEvent::Attacks }
    #[must_use]
    pub const fn blocks(self) -> abilities::TriggerEvent { abilities::TriggerEvent::Blocks }
    #[must_use]
    pub const fn becomes_blocked(self) -> abilities::TriggerEvent {
        abilities::TriggerEvent::BecomesBlocked
    }
    #[must_use]
    pub fn deals<D: Dealt>(self, dealt: D) -> D::Builder { dealt.build() }
}

#[cfg(test)]
mod tests {
    use crate::PLAYER;
    use crate::abilities;
    use crate::rules;

    use super::This;

    #[test]
    fn test_usage() {
        assert_eq!(This.dies(), abilities::TriggerEvent::Dies);
        assert_eq!(This.attacks(), abilities::TriggerEvent::Attacks);
        assert_eq!(This.blocks(), abilities::TriggerEvent::Blocks);
        assert_eq!(
            This.becomes_blocked(),
            abilities::TriggerEvent::BecomesBlocked
        );
        assert_eq!(
            This.deals(rules::COMBAT_DAMAGE).to(rules::a(PLAYER)),
            abilities::TriggerEvent::DealsDamage {
                combat: true,
                to: rules::Players::All,
            }
        );
    }
}
