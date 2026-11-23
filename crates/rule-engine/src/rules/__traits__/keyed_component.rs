use bevy::prelude::Component;

use crate::rules;

pub trait KeyedComponent: Component {
    const KEY: rules::ComponentKey;
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::rules;
    use crate::rules::traits::keyed_component::KeyedComponent;

    #[test]
    fn test_usage() {
        assert_eq!(cards::Tapped::KEY, rules::ComponentKey("tapped"));
        assert_eq!(cards::Attacking::KEY, rules::ComponentKey("attacking"));
        assert_eq!(cards::Flying::KEY, rules::ComponentKey("flying"));
        assert_eq!(cards::Reach::KEY, rules::ComponentKey("reach"));
        assert_eq!(cards::Vigilance::KEY, rules::ComponentKey("vigilance"));
        assert_eq!(cards::Defender::KEY, rules::ComponentKey("defender"));
        assert_eq!(
            cards::SummoningSick::KEY,
            rules::ComponentKey("summoning-sick")
        );
    }
}
