#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EffectKind {
    Damage(u32),
    Destroy,
    Counter,
    Draw(u32),
    LoseLife(u32),
    Discard(u32),
    ModifyPowerToughness { power: i32, toughness: i32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FilterKind {
    Creature,
    Player,
    AnyTarget,
    Spell,
    CreatureSpell,
    NoncreatureSpell,
    InstantSpell,
}
