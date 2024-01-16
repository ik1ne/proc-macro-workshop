use bitfield::*;

#[derive(BitfieldSpecifier)]
pub enum TriggerMode {
    Edge = 0,
    Level = 1,
}

fn main() {}
