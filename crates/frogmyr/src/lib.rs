extern crate self as frogmyr;

#[path = "__basic__/mod.rs"]
mod basic;
mod case;
pub(crate) mod compiled_rule;
mod constants;
mod facts;
mod program;
pub(crate) mod wired_rule;

pub use basic::atom::Atom;
pub use basic::cmp_op::CmpOp;
pub use basic::engine_error::EngineError;
pub use basic::literal::Literal;
pub use basic::payload::Payload;
pub use basic::rel::Rel;
pub use basic::rule::Rule;
pub use basic::term::Term;
pub use basic::value::Value;
pub use case::Case;
pub(crate) use compiled_rule::CompiledRule;
pub(crate) use constants::COUNT_RELATION;
pub use facts::Facts;
pub use frogmyr_macros::rules;
pub use program::Program;
pub(crate) use wired_rule::WiredRule;

#[path = "../__methods__/mod.rs"]
mod methods;

#[cfg(test)]
mod tests {
    use crate::Case;

    #[test]
    fn test_usage() {
        assert!(Case::cases().iter().any(|case| case.name == "rules_macro_builds_ir"));
    }
}
