use crate::basic::structs::AtomSpec;

pub enum CmpOpSpec {
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

pub enum ConjunctSpec {
    Pos(AtomSpec),
    Neg(AtomSpec),
    Count { output: String, body: Vec<AtomSpec> },
    Cmp(TermSpec, CmpOpSpec, TermSpec),
}

pub enum TermSpec {
    Var(String),
    Sym(String),
    Const(proc_macro2::TokenStream),
    Int(i64),
}
