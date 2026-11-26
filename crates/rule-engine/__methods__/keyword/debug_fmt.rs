use crate::keywords;

pub fn debug_fmt(
    keyword: &keywords::Keyword,
    formatter: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    formatter
        .debug_struct("Keyword")
        .field("id", &keyword.id)
        .field("params", &keyword.params)
        .finish_non_exhaustive()
}

// no test_usage necessary
