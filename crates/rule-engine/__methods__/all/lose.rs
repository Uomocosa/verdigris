use crate::abilities;
use crate::effects;
use crate::keywords;
use crate::statics;

pub fn lose(all: statics::All, keyword: keywords::StaticKeyword) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        modification: effects::Modification::Characteristic(effects::Layered::LoseKeyword(keyword)),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::keywords;
    use crate::statics;

    use super::lose;

    #[test]
    fn test_usage() {
        let ability = lose(statics::all(CREATURE), keywords::flying());
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.modification,
            effects::Modification::Characteristic(effects::Layered::LoseKeyword(
                keywords::StaticKeyword::Flying
            ))
        );
    }
}
