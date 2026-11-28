use verdigris_datalog_ir::Atom;
use verdigris_datalog_ir::Rel;
use verdigris_datalog_ir::Term;

use crate::PlayDatalog;

pub fn holds_head(index: usize) -> Atom {
    Atom {
        rel: Rel(Box::leak(format!("holds_{index}").into_boxed_str())),
        terms: vec![
            Term::Var(PlayDatalog::CARD_VAR),
            Term::Var(PlayDatalog::PLAYER_VAR),
        ],
    }
}

#[cfg(test)]
mod tests {
    use verdigris_datalog_ir::Atom;
    use verdigris_datalog_ir::Rel;
    use verdigris_datalog_ir::Term;

    use super::holds_head;
    use crate::PlayDatalog;

    #[test]
    fn test_usage() {
        assert_eq!(
            holds_head(2),
            Atom {
                rel: Rel("holds_2"),
                terms: vec![
                    Term::Var(PlayDatalog::CARD_VAR),
                    Term::Var(PlayDatalog::PLAYER_VAR),
                ],
            }
        );
    }
}
