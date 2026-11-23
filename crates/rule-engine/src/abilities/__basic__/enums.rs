#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Replacement {
    EntersTapped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ActivationCost {
    Tap,
}
