use frogmyr::Atom;
use frogmyr::Rel;
use frogmyr::Term;

use crate::legality;

pub fn holds_head(index: usize) -> Result<Atom, legality::TranslationError> {
    let name = legality::PlayDatalog::HOLDS_NAMES
        .get(index)
        .copied()
        .ok_or(legality::TranslationError::TooManyConditions(index))?;
    Ok(Atom {
        rel: Rel(name),
        terms: vec![
            Term::Var(legality::PlayDatalog::CARD_VAR),
            Term::Var(legality::PlayDatalog::PLAYER_VAR),
        ],
    })
}

#[cfg(test)]
mod tests {
    use crate::legality;
    use frogmyr::Atom;
    use frogmyr::Rel;
    use frogmyr::Term;

    use super::holds_head;

    #[test]
    fn test_usage() {
        assert_eq!(
            holds_head(2).unwrap(),
            Atom {
                rel: Rel("holds_2"),
                terms: vec![
                    Term::Var(legality::PlayDatalog::CARD_VAR),
                    Term::Var(legality::PlayDatalog::PLAYER_VAR),
                ],
            }
        );
    }

    #[test]
    fn index_past_the_table_is_an_explicit_error() {
        assert_eq!(
            holds_head(legality::PlayDatalog::HOLDS_NAMES.len()),
            Err(legality::TranslationError::TooManyConditions(
                legality::PlayDatalog::HOLDS_NAMES.len()
            ))
        );
        assert!(holds_head(legality::PlayDatalog::HOLDS_NAMES.len() - 1).is_ok());
    }
}
