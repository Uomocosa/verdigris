use derive_more::Deref;

use crate::keywords;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct KeywordId(pub &'static str);

#[rustfmt::skip]
impl From<keywords::Keyword> for KeywordId {
    fn from(keyword: keywords::Keyword) -> Self { keyword.id }
}
