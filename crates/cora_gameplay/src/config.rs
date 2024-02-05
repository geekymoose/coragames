// TODO This is to improve (for now, just easy WIP)
pub const DEFAULT_UNIT_VISION_RANGE: usize = 5;
pub const DEFAULT_UNIT_STRENGTH: u32 = 10;
pub const DEFAULT_UNIT_HEALTH: u32 = 100;

#[derive(Debug)]
pub struct GridConfig {
    pub width: usize,
    pub height: usize,
}
