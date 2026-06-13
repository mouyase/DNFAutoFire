#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TapConfig {
    pub rate_per_second: u16,
    pub down_ms: u16,
    pub gap_ms: u16,
}

impl Default for TapConfig {
    fn default() -> Self {
        Self {
            rate_per_second: 60,
            down_ms: 8,
            gap_ms: 8,
        }
    }
}
