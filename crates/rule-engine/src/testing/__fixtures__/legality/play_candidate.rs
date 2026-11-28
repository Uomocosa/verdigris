use frogmyr::Atom;
use frogmyr::Term;

use crate::legality;

#[must_use]
pub fn play_candidate() -> Atom {
    Atom {
        rel: legality::PlayDatalog::PLAY_CANDIDATE,
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

    use super::play_candidate;
    use crate::legality;

    #[test]
    fn test_usage() {
        assert_eq!(
            play_candidate(),
            Atom {
                rel: legality::PlayDatalog::PLAY_CANDIDATE,
                terms: vec![
                    Term::Var(legality::PlayDatalog::CARD_VAR),
                    Term::Var(legality::PlayDatalog::PLAYER_VAR),
                ],
            }
        );
    }
}
