use frogmyr::Atom;
use frogmyr::Term;

use crate::legality;

#[must_use]
pub fn block_candidate() -> Atom {
    Atom {
        rel: legality::BlockDatalog::BLOCK_CANDIDATE,
        terms: vec![
            Term::Var(legality::PlayDatalog::CARD_VAR),
            Term::Var(legality::PlayDatalog::PLAYER_VAR),
        ],
    }
}

#[cfg(test)]
mod tests {
    use frogmyr::Atom;
    use frogmyr::Term;

    use super::block_candidate;
    use crate::legality;

    #[test]
    fn test_usage() {
        assert_eq!(
            block_candidate(),
            Atom {
                rel: legality::BlockDatalog::BLOCK_CANDIDATE,
                terms: vec![
                    Term::Var(legality::PlayDatalog::CARD_VAR),
                    Term::Var(legality::PlayDatalog::PLAYER_VAR),
                ],
            }
        );
    }
}
