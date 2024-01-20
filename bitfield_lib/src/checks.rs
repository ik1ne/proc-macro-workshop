pub enum ZeroMod8 {}
pub enum OneMod8 {}
pub enum TwoMod8 {}
pub enum ThreeMod8 {}
pub enum FourMod8 {}
pub enum FiveMod8 {}
pub enum SixMod8 {}
pub enum SevenMod8 {}

pub trait GetMarker {
    type Marker;
}

impl GetMarker for [(); 0] {
    type Marker = ZeroMod8;
}

impl GetMarker for [(); 1] {
    type Marker = OneMod8;
}

impl GetMarker for [(); 2] {
    type Marker = TwoMod8;
}

impl GetMarker for [(); 3] {
    type Marker = ThreeMod8;
}

impl GetMarker for [(); 4] {
    type Marker = FourMod8;
}

impl GetMarker for [(); 5] {
    type Marker = FiveMod8;
}

impl GetMarker for [(); 6] {
    type Marker = SixMod8;
}

impl GetMarker for [(); 7] {
    type Marker = SevenMod8;
}

pub trait TotalSizeIsMultipleOfEightBits {
    type _Check;
}

impl TotalSizeIsMultipleOfEightBits for ZeroMod8 {
    type _Check = ();
}

pub type MultipleOfEight<T> = <<T as GetMarker>::Marker as TotalSizeIsMultipleOfEightBits>::_Check;
