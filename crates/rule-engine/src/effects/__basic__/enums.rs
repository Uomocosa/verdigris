#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectKind {
    Damage(u32),
    Destroy,
    Counter,
    Draw(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilterKind {
    Creature,
    AnyTarget,
    Spell,
}
