use derive_more::Deref;
use derive_more::DerefMut;

#[derive(Deref, DerefMut)]
pub struct HasherProxy<'a>(pub(crate) &'a mut dyn std::hash::Hasher);

#[rustfmt::skip]
impl std::hash::Hasher for HasherProxy<'_> {
    fn finish(&self) -> u64 { (**self).finish() }
    fn write(&mut self, bytes: &[u8]) { (**self).write(bytes); }
}

#[cfg(test)]
mod tests {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hash;
    use std::hash::Hasher;

    use super::HasherProxy;

    #[test]
    fn test_usage() {
        let mut direct = DefaultHasher::new();
        42u8.hash(&mut direct);
        let mut proxied = DefaultHasher::new();
        42u8.hash(&mut HasherProxy(&mut proxied));
        assert_eq!(direct.finish(), proxied.finish());
    }
}
