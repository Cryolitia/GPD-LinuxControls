#[derive(Debug,Copy, Clone, Default)]
pub struct DeadZonesConfig{
    pub left: SpecificDeadZone,
    pub right: SpecificDeadZone
}

#[derive(Debug, Copy, Clone, Default)]
pub struct SpecificDeadZone {
    pub center: i8,
    pub border: i8,
}