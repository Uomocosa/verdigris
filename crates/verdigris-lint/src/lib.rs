mod constants;
pub use constants::{EXEMPTIONS, RULES_ROOT};
#[path = "../__methods__/mod.rs"]
mod methods;
pub mod rules;
mod verdigris_linter;
pub use verdigris_linter::VerdigrisLinter;
