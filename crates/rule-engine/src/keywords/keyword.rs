use std::sync::Arc;

use atomic_delegate_macros::atomic_delegate;

use crate::effects;
use crate::keywords;
use crate::keywords::traits::params::Params;

#[derive(Clone)]
pub struct Keyword {
    pub id: keywords::KeywordId,
    pub(crate) params: Arc<dyn Params>,
    pub(crate) grants: Arc<dyn Fn() -> Vec<effects::Modification> + Send + Sync>,
}

impl Keyword {
    #[must_use]
    pub fn from_plain(
        id: keywords::KeywordId,
        abilities: fn() -> Vec<effects::Modification>,
    ) -> Self {
        Self {
            id,
            params: Arc::new(()),
            grants: Arc::new(abilities),
        }
    }

    #[must_use]
    pub fn from_with<P>(
        id: keywords::KeywordId,
        params: P,
        abilities: fn(&P) -> Vec<effects::Modification>,
    ) -> Self
    where
        P: Params + Clone,
    {
        Self {
            id,
            params: Arc::new(params.clone()),
            grants: Arc::new(move || abilities(&params)),
        }
    }
}

#[rustfmt::skip]
impl PartialEq for Keyword {
    fn eq(&self, other: &Self) -> bool { self.id == other.id && self.params.dyn_eq(other.params.as_ref()) }
}

impl Eq for Keyword {}

impl std::hash::Hash for Keyword {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.params.dyn_hash(state);
    }
}

impl std::fmt::Debug for Keyword {
    #[atomic_delegate(Keyword, Debug)]
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {}
}

#[rustfmt::skip]
impl Keyword {
    #[must_use]
    pub fn abilities(&self) -> Vec<effects::Modification> { (self.grants)() }

    #[must_use]
    pub fn params<P: 'static>(&self) -> Option<&P> { self.params.as_any().downcast_ref::<P>() }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::keywords;

    #[test]
    fn test_usage() {
        let keyword = keywords::flying();
        assert_eq!(keyword.id, keywords::Flying::ID);
        assert_eq!(keyword.abilities(), Vec::new());
        assert_eq!(keyword.params::<()>(), Some(&()));
        let enchanted = keywords::enchant(CREATURE);
        assert_eq!(enchanted.id, keywords::Enchant::ID);
        assert_eq!(
            enchanted.params::<keywords::EnchantFilter>(),
            Some(&keywords::EnchantFilter(CREATURE))
        );
        assert_eq!(enchanted, keywords::enchant(CREATURE));
        assert_ne!(enchanted, keyword);
    }
}
