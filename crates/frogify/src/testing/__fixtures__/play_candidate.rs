use frogmyr::Atom;
use frogmyr::Term;

use crate::PlayDatalog;

#[must_use]
pub fn play_candidate() -> Atom {
    Atom {
        rel: PlayDatalog::PLAY_CANDIDATE,
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

    use super::play_candidate;
    use crate::PlayDatalog;

    #[test]
    fn test_usage() {
        assert_eq!(
            play_candidate(),
            Atom {
                rel: PlayDatalog::PLAY_CANDIDATE,
                terms: vec![
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Var(PlayDatalog::PLAYER_VAR),
                ],
            }
        );
    }
}
