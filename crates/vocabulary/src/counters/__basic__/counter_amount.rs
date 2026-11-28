use crate::counters;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CounterAmount {
    pub count: u32,
    pub kind: counters::CounterId,
}

impl std::ops::Mul<counters::CounterId> for u32 {
    type Output = counters::CounterAmount;

    fn mul(self, kind: counters::CounterId) -> counters::CounterAmount {
        counters::CounterAmount { count: self, kind }
    }
}
