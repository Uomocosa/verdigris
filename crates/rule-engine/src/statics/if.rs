use derive_more::Deref;

use crate::rules;
use crate::statics;

#[derive(Debug, Clone, PartialEq, Eq, Deref)]
pub struct If(pub statics::All);

#[rustfmt::skip]
impl If {
    #[must_use]
    pub fn would<E: rules::traits::event::Event>(self) -> statics::Would<E> {
        statics::Would::of(&*self)
    }
}

#[cfg(test)]
mod tests {
    use crate::creature;
    use crate::events;
    use crate::rules::traits::event::Event;
    use crate::statics;

    #[test]
    fn test_usage() {
        let conditional = statics::if_a(creature());
        assert_eq!(*conditional, statics::all(creature()));
        let would: statics::Would<events::Draw> = conditional.would();
        assert_eq!(would, statics::Would::of(creature()));
        assert_eq!(would.holder, events::Draw::HOLDER);
    }
}
