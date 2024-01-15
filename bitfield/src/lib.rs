pub use bitfield_derive::bitfield;
use bitfield_derive::generate_bitfield_types;
pub use bitfield_lib::Specifier;

pub mod checks {
    pub use bitfield_lib::checks::*;
}

generate_bitfield_types!();
