use crate::abilities;
use crate::effects;
use crate::keywords;
use crate::statics;

pub fn gain(all: statics::All, keyword: keywords::Keyword) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Characteristic(effects::Layered::GainKeyword(keyword)),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::keywords;
    use crate::statics;

    use super::gain;

    #[test]
    fn test_usage() {
        let ability = gain(statics::all(CREATURE), keywords::vigilance());
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Characteristic(effects::Layered::GainKeyword(
                keywords::vigilance()
            ))
        );
    }
}
