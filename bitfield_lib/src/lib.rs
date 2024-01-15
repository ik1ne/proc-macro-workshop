mod bitfield;
pub mod check;
mod generate;

pub use bitfield::bitfield_inner;
pub use generate::generate_token_stream_for_bitfield_types;

pub trait Specifier {
    const BITS: usize;
    type BitFieldType;

    fn get(arr: &[u8], offset: usize) -> Self::BitFieldType;
    fn set(arr: &mut [u8], offset: usize, value: Self::BitFieldType);
}
