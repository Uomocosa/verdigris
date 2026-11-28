use frogmyr::Atom;
use frogmyr::Term;

use crate::BlockDatalog;
use crate::PlayDatalog;

#[must_use]
pub fn block_candidate() -> Atom {
    Atom {
        rel: BlockDatalog::BLOCK_CANDIDATE,
        terms: vec![
            Term::Var(PlayDatalog::CARD_VAR),
            Term::Var(PlayDatalog::PLAYER_VAR),
        ],
    }
}

#[cfg(test)]
mod tests {
    use frogmyr::Atom;
    use frogmyr::Term;

    use super::block_candidate;
    use crate::BlockDatalog;
    use crate::PlayDatalog;

    #[test]
    fn test_usage() {
        assert_eq!(
            block_candidate(),
            Atom {
                rel: BlockDatalog::BLOCK_CANDIDATE,
                terms: vec![
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Var(PlayDatalog::PLAYER_VAR),
                ],
            }
        );
    }
}
