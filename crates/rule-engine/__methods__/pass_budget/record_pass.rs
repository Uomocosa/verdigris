use crate::testing;

pub const fn record_pass(budget: &mut testing::PassBudget) -> Result<(), testing::PlayError> {
    let testing::PassBudget(passes) = budget;
    *passes = passes.saturating_sub(1);
    if *passes == 0 {
        return Err(testing::PlayError::Stalled);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::testing;

    use super::record_pass;

    #[test]
    fn test_usage() {
        let mut budget = testing::PassBudget::of(1);

        assert_eq!(record_pass(&mut budget), Err(testing::PlayError::Stalled));
    }
}
