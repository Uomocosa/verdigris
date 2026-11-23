use std::marker::PhantomData;

use atomic_delegate_macros::atomic_delegates;

use crate::rules;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ClauseId<Ctx> {
    pub key: rules::ClauseKey,
    tag: PhantomData<Ctx>,
}

impl<Ctx> ClauseId<Ctx> {
    #[must_use]
    pub const fn new(key: &'static str) -> Self {
        Self {
            key: rules::ClauseKey(key),
            tag: PhantomData,
        }
    }
}

#[rustfmt::skip]
impl<Ctx> Clone for ClauseId<Ctx> {
    fn clone(&self) -> Self { *self }
}

impl<Ctx> Copy for ClauseId<Ctx> {}

#[atomic_delegates]
impl<Ctx> ClauseId<Ctx> {
    #[must_use]
    pub fn clause(
        self,
        text: &'static str,
        role: rules::Role,
        condition: rules::Predicate,
    ) -> rules::Clause {
    }
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;

    use super::ClauseId;

    #[test]
    fn test_usage() {
        let id = ClauseId::<rules::BlockCtx>::new("509.1a/untapped");
        let built = id.clause(
            "The chosen creatures must be untapped",
            rules::Role::Subject,
            !rules::has::<cards::Tapped>(),
        );
        assert_eq!(built.key, rules::ClauseKey("509.1a/untapped"));
        assert_eq!(built.text, "The chosen creatures must be untapped");
        assert_eq!(built.role, rules::Role::Subject);
        assert_eq!(built.condition, !rules::has::<cards::Tapped>());
    }
}
