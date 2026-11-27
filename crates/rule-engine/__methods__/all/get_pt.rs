use crate::abilities;
use crate::effects;
use crate::statics;

pub fn get_pt(all: &statics::All, power: i32, toughness: i32) -> abilities::Continuous {
    let filter: &effects::Filter = all;
    abilities::Continuous {
        filter: filter.clone(),
        as_long_as: None,
        modification: effects::Modification::Characteristic(effects::Layered::ModifyPT {
            power,
            toughness,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::creature;
    use crate::effects;
    use crate::statics;

    use super::get_pt;

    #[test]
    fn test_usage() {
        let ability = get_pt(&statics::all(creature()), 1, 1);
        assert_eq!(ability.filter, creature());
        assert_eq!(
            ability.modification,
            effects::Modification::Characteristic(effects::Layered::ModifyPT {
                power: 1,
                toughness: 1,
            })
        );
    }
}
