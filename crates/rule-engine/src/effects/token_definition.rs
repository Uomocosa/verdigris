use std::hash::{Hash, Hasher};

use derive_more::{Deref, From};

use crate::cards;

#[derive(Clone, Copy, Debug, Deref, From)]
pub struct TokenDefinition(fn() -> cards::CardDefinition);

#[rustfmt::skip]
impl PartialEq for TokenDefinition {
    fn eq(&self, other: &Self) -> bool { self.definition() == other.definition() }
}

impl Eq for TokenDefinition {}

#[rustfmt::skip]
impl Hash for TokenDefinition {
    fn hash<H: Hasher>(&self, state: &mut H) { self.definition().name.hash(state); }
}

#[rustfmt::skip]
impl TokenDefinition {
    #[must_use]
    pub fn definition(self) -> cards::CardDefinition { (*self)() }
}

#[cfg(test)]
mod tests {
    use crate::testing;

    use super::TokenDefinition;

    #[test]
    fn test_usage() {
        let token = TokenDefinition::from(testing::SOLDIER_TOKEN.definition);

        assert_eq!(*token.definition().name, "Soldier");
        assert_eq!(
            token,
            TokenDefinition::from(testing::SOLDIER_TOKEN.definition)
        );
    }
}
