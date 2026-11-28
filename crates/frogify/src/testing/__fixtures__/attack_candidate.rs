use frogmyr::Atom;
use frogmyr::Term;

use crate::AttackDatalog;
use crate::PlayDatalog;

#[must_use]
pub fn attack_candidate() -> Atom {
    Atom {
        rel: AttackDatalog::ATTACK_CANDIDATE,
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

    use super::attack_candidate;
    use crate::AttackDatalog;
    use crate::PlayDatalog;

    #[test]
    fn test_usage() {
        assert_eq!(
            attack_candidate(),
            Atom {
                rel: AttackDatalog::ATTACK_CANDIDATE,
                terms: vec![
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Var(PlayDatalog::PLAYER_VAR),
                ],
            }
        );
    }
}
