use crate::counters;
use crate::effects;
use crate::effects::traits::gets::Gets;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CounterAmount {
    pub count: u32,
    pub kind: counters::CounterId,
}

#[rustfmt::skip]
impl Gets for CounterAmount {
    fn kind(self) -> effects::EffectKind { effects::EffectKind::Poison(self) }
}

impl std::ops::Mul<counters::CounterId> for u32 {
    type Output = counters::CounterAmount;

    fn mul(self, kind: counters::CounterId) -> counters::CounterAmount {
        counters::CounterAmount { count: self, kind }
    }
}
