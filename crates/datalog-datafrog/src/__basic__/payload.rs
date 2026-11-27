use verdigris_datalog_ir::Value;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Payload {
    pub alive: bool,
    pub bindings: Vec<Option<Value>>,
}
