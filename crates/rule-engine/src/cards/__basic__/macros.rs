#[macro_export]
macro_rules! mana_symbol {
    (W) => {
        $crate::cards::ManaSymbol::White
    };
    (U) => {
        $crate::cards::ManaSymbol::Blue
    };
    (B) => {
        $crate::cards::ManaSymbol::Black
    };
    (R) => {
        $crate::cards::ManaSymbol::Red
    };
    (G) => {
        $crate::cards::ManaSymbol::Green
    };
    ($amount:literal) => {
        $crate::cards::ManaSymbol::Generic($amount)
    };
}

#[macro_export]
macro_rules! cost {
    ($($symbol:tt),* $(,)?) => {
        $crate::cards::Cost::default()$(.plus($crate::mana_symbol!($symbol)))*
    };
}

#[macro_export]
macro_rules! mana {
    ($($symbol:tt),* $(,)?) => {
        $crate::cards::ManaPool::EMPTY$(.plus($crate::mana_symbol!($symbol)))*
    };
}

#[macro_export]
macro_rules! pt {
    ($power:expr, $toughness:expr $(,)?) => {
        $crate::cards::PowerToughness {
            power: $power,
            toughness: $toughness,
        }
    };
}

#[macro_export]
macro_rules! card {
    ($base:expr; $($ability:expr),* $(,)?) => {
        ($base).abilities([$($crate::abilities::Ability::from($ability)),*])
    };
}
