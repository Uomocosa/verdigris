#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Phase {
    Untap,
    Upkeep,
    Draw,
    PrecombatMain,
    Combat,
    PostcombatMain,
    End,
}
