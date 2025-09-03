use super::*;

impl CoinTreasury {
    pub const fn const_default() -> Self {
        Self {
            id: None,
            total_supply: None,
            supply_state: None,
        }
    }
}
