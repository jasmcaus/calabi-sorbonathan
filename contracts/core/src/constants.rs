pub const PRECISION: u128 = 1e5 as u128;
pub const RATE_PRECISION: u128 = 1e18 as u128;
pub const LIQUIDATION_THRESHOLD: u128 = 8e4 as u128; // 80%
pub const LIQUIDATION_CLOSE_FACTOR: u128 = 5e4 as u128; // 50%
pub const LIQUIDATION_REWARD: u128 = 5e3 as u128; // 5%
pub const MIN_HEALTH_FACTOR: u128 = 1e18 as u128;

// Default Interest Rate (if borrows = 0)
pub const DEFAULT_INTEREST: u64 = 158247046; // 0.5% annual rate 1e18 precision

// Protocol Fee (1e5 precision)
pub const DEFAULT_PROTOCOL_FEE: u128 = 0;
pub const MAX_PROTOCOL_FEE: u128 = 1e4 as u128; // 10%

pub const BLOCKS_PER_YEAR: u128 = 2102400; // Average Ethereum blocks per year
