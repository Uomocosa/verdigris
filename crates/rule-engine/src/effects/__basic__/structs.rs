use crate::effects;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Target {
    pub count: u32,
    pub filter: effects::Filter,
}
