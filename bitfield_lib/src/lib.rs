mod bitfield;
mod generate;

pub use bitfield::bitfield_inner;
pub use generate::generate_token_stream_for_bitfield_types;

pub trait Specifier {
    const BITS: usize;
}
