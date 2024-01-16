pub use bitfield::bitfield_inner;
pub use specifier::generate::bitfield_type::generate_token_stream_for_bitfield_types;
pub use specifier::generate::derive_bitfield_specifier;
pub use specifier::Specifier;

mod bitfield;
pub mod checks;
mod specifier;
