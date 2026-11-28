use verdigris_datalog_ir::Atom;
use verdigris_datalog_ir::Rel;
use verdigris_datalog_ir::Term;

use crate::PlayDatalog;
use crate::TranslationError;

pub fn holds_head(index: usize) -> Result<Atom, TranslationError> {
    let name = PlayDatalog::HOLDS_NAMES
        .get(index)
        .copied()
        .ok_or(TranslationError::TooManyConditions(index))?;
    Ok(Atom {
        rel: Rel(name),
        terms: vec![
            Term::Var(PlayDatalog::CARD_VAR),
            Term::Var(PlayDatalog::PLAYER_VAR),
        ],
    })
}

#[cfg(test)]
mod tests {
    use verdigris_datalog_ir::Atom;
    use verdigris_datalog_ir::Rel;
    use verdigris_datalog_ir::Term;

    use super::holds_head;
    use crate::PlayDatalog;
    use crate::TranslationError;

    #[test]
    fn test_usage() {
        assert_eq!(
            holds_head(2).unwrap(),
            Atom {
                rel: Rel("holds_2"),
                terms: vec![
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Var(PlayDatalog::PLAYER_VAR),
                ],
            }
        );
    }

    #[test]
    fn index_past_the_table_is_an_explicit_error() {
        assert_eq!(
            holds_head(PlayDatalog::HOLDS_NAMES.len()),
            Err(TranslationError::TooManyConditions(
                PlayDatalog::HOLDS_NAMES.len()
            ))
        );
        assert!(holds_head(PlayDatalog::HOLDS_NAMES.len() - 1).is_ok());
    }
}
