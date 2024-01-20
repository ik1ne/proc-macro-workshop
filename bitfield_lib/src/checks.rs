pub enum ZeroMod8 {}
pub enum OneMod8 {}
pub enum TwoMod8 {}
pub enum ThreeMod8 {}
pub enum FourMod8 {}
pub enum FiveMod8 {}
pub enum SixMod8 {}
pub enum SevenMod8 {}

pub trait GetModular {
    type Marker;
}

impl GetModular for [(); 0] {
    type Marker = ZeroMod8;
}

impl GetModular for [(); 1] {
    type Marker = OneMod8;
}

impl GetModular for [(); 2] {
    type Marker = TwoMod8;
}

impl GetModular for [(); 3] {
    type Marker = ThreeMod8;
}

impl GetModular for [(); 4] {
    type Marker = FourMod8;
}

impl GetModular for [(); 5] {
    type Marker = FiveMod8;
}

impl GetModular for [(); 6] {
    type Marker = SixMod8;
}

impl GetModular for [(); 7] {
    type Marker = SevenMod8;
}

pub trait TotalSizeIsMultipleOfEightBits {
    type _Check;
}

impl TotalSizeIsMultipleOfEightBits for ZeroMod8 {
    type _Check = ();
}

pub type MultipleOfEight<T> = <<T as GetModular>::Marker as TotalSizeIsMultipleOfEightBits>::_Check;

pub type InRange<T> = <<T as GetBool>::Marker as DiscriminantInRange>::_Check;

pub trait GetBool {
    type Marker;
}

pub enum True {}
pub enum False {}

impl GetBool for [(); 1] {
    type Marker = True;
}

impl GetBool for [(); 0] {
    type Marker = False;
}

pub trait DiscriminantInRange {
    type _Check;
}

impl DiscriminantInRange for True {
    type _Check = ();
}
