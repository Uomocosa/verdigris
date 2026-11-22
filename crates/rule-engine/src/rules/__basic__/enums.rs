#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Question {
    TapsWhenAttacking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Answer {
    TapsWhenAttacking(bool),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Combine {
    LatestTimestamp,
}
