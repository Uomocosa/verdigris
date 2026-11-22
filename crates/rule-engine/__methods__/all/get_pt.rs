use crate::abilities;
use crate::effects;
use crate::statics;

pub fn get_pt(all: statics::All, power: i32, toughness: i32) -> abilities::Continuous {
    abilities::Continuous {
        filter: *all,
        change: effects::Change::Characteristic(effects::Layered::ModifyPT { power, toughness }),
    }
}

#[cfg(test)]
mod tests {
    use crate::CREATURE;
    use crate::effects;
    use crate::statics;

    use super::get_pt;

    #[test]
    fn test_usage() {
        let ability = get_pt(statics::all(CREATURE), 1, 1);
        assert_eq!(ability.filter, CREATURE);
        assert_eq!(
            ability.change,
            effects::Change::Characteristic(effects::Layered::ModifyPT {
                power: 1,
                toughness: 1,
            })
        );
    }
}
