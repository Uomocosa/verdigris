use crate::effects;
use crate::rules;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Target {
    pub count: u32,
    pub filter: effects::Filter,
}

impl From<rules::This> for Target {
    fn from(_: rules::This) -> Self {
        Self {
            count: 1,
            filter: effects::Filter {
                kind: effects::FilterKind::Creature,
                controlled_by: None,
                excluded_color: None,
                attacking: false,
                nontoken: false,
                object: Some(effects::ObjectRelation::ThisObject),
            },
        }
    }
}
