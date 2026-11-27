#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Count {
    pub min: u32,
    pub max: u32,
}

impl Count {
    #[must_use]
    pub const fn exactly(count: u32) -> Self {
        Self {
            min: count,
            max: count,
        }
    }

    #[must_use]
    pub const fn up_to(count: u32) -> Self {
        Self { min: 0, max: count }
    }
}

#[cfg(test)]
mod tests {
    use super::Count;

    #[test]
    fn test_usage() {
        assert_eq!(Count::exactly(2), Count { min: 2, max: 2 });
        assert_eq!(Count::up_to(2), Count { min: 0, max: 2 });
    }
}
