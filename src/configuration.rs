#[allow(dead_code)]
pub struct SimbarConfig {
    pub width_fallback: u32,
    pub width: Option<u32>,
    pub height: u32,
    pub primary_output: Option<&'static str>,
    pub frame_rate: u64,
}

pub const SIMBAR_CONFIG: SimbarConfig = SimbarConfig {
    width_fallback: 1920,
    width: None,
    height: 40,
    primary_output: None,
    frame_rate: 20,
};
