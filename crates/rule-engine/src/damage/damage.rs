use bevy::prelude::Component;
use derive_more::Deref;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deref)]
pub struct Damage(pub u32);

#[rustfmt::skip]
impl Damage {
    #[must_use]
    pub fn signed_amount(amount: u32) -> i32 { i32::try_from(amount).unwrap_or(i32::MAX) }
}

#[cfg(test)]
mod tests {
    use super::Damage;

    #[test]
    fn test_usage() {
        assert_eq!(Damage::signed_amount(3), 3);
        assert_eq!(Damage::signed_amount(u32::MAX), i32::MAX);
    }
}
