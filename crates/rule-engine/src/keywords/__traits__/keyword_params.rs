use std::any::Any;
use std::fmt::Debug;
use std::hash::Hash;

use crate::keywords;

pub trait KeywordParams: Any + Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn dyn_eq(&self, other: &dyn KeywordParams) -> bool;
    fn dyn_hash(&self, state: &mut dyn std::hash::Hasher);
}

impl<T> KeywordParams for T
where
    T: Any + Debug + Eq + Hash + Send + Sync,
{
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dyn_eq(&self, other: &dyn KeywordParams) -> bool {
        other.as_any().downcast_ref::<T>() == Some(self)
    }

    fn dyn_hash(&self, state: &mut dyn std::hash::Hasher) {
        Hash::hash(self, &mut keywords::HasherProxy(state));
    }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;

    use crate::CREATURE;
    use crate::keywords;
    use crate::keywords::traits::keyword_params::KeywordParams;

    #[test]
    fn test_usage() {
        let plain: &dyn KeywordParams = &();
        assert!(plain.dyn_eq(&()));
        let filtered: &dyn KeywordParams = &keywords::EnchantFilter(CREATURE);
        assert!(filtered.dyn_eq(&keywords::EnchantFilter(CREATURE)));
        assert!(!plain.dyn_eq(filtered));
        let mut dynamic = DefaultHasher::new();
        plain.dyn_hash(&mut dynamic);
        assert_eq!(dynamic.finish(), DefaultHasher::new().finish());
        let mut dynamic = DefaultHasher::new();
        filtered.dyn_hash(&mut dynamic);
        let mut direct = DefaultHasher::new();
        keywords::EnchantFilter(CREATURE).hash(&mut direct);
        assert_eq!(dynamic.finish(), direct.finish());
    }
}
