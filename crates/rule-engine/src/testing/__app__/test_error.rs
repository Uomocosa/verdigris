use crate::testing;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum TestError {
    #[error(transparent)]
    Cards(#[from] testing::CardsError),
    #[error(transparent)]
    Play(#[from] testing::PlayError),
}
