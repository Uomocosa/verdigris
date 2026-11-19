use crate::cards;

pub fn pay(pool: &mut cards::ManaPool, cost: cards::Cost) -> bool {
    let affordable = pool.white >= u32::from(cost.white)
        && pool.blue >= u32::from(cost.blue)
        && pool.black >= u32::from(cost.black)
        && pool.red >= u32::from(cost.red)
        && pool.green >= u32::from(cost.green)
        && pool.colorless >= cost.generic;
    if !affordable {
        return false;
    }
    pool.white = pool.white.saturating_sub(u32::from(cost.white));
    pool.blue = pool.blue.saturating_sub(u32::from(cost.blue));
    pool.black = pool.black.saturating_sub(u32::from(cost.black));
    pool.red = pool.red.saturating_sub(u32::from(cost.red));
    pool.green = pool.green.saturating_sub(u32::from(cost.green));
    pool.colorless = pool.colorless.saturating_sub(cost.generic);
    true
}

#[cfg(test)]
mod tests {
    use crate::cards;
    use crate::cost;
    use crate::mana;

    use super::pay;

    #[test]
    fn test_usage() {
        let mut pool = mana![1, B, B];
        assert!(pay(&mut pool, cost![1, B, B]));
        assert_eq!(pool, cards::ManaPool::default());
        assert!(!pay(&mut pool, cost![B]));
    }
}
