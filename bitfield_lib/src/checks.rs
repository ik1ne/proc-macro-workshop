/// ```rust
/// # use bitfield_lib::checks::*;
/// // test: (0 + 1) + 7 == impl TotalSizeIsMultipleOfEightBits
/// fn _test() -> impl TotalSizeIsMultipleOfEightBits {
///     let _zero: <<ZeroMod8 as AddMod8<OneMod8>>::Output as AddMod8<SevenMod8>>::Output = ZeroMod8;
///     _zero
/// }
/// ```
/// ```compile_fail
/// # use bitfield_lib::check::*;
/// // test: 0 + 1 != impl TotalSizeIsMultipleOfEightBits
/// fn _test() -> impl TotalSizeIsMultipleOfEightBits {
///     let _one: <ZeroMod8 as AddMod8<OneMod8>>::Output = OneMod8;
///     _one
/// }
/// ```
pub trait TotalSizeIsMultipleOfEightBits {}

pub trait Modulo {}

pub struct ZeroMod8;
pub struct OneMod8;
pub struct TwoMod8;
pub struct ThreeMod8;
pub struct FourMod8;
pub struct FiveMod8;
pub struct SixMod8;
pub struct SevenMod8;

impl Modulo for ZeroMod8 {}
impl Modulo for OneMod8 {}
impl Modulo for TwoMod8 {}
impl Modulo for ThreeMod8 {}
impl Modulo for FourMod8 {}
impl Modulo for FiveMod8 {}
impl Modulo for SixMod8 {}
impl Modulo for SevenMod8 {}

impl TotalSizeIsMultipleOfEightBits for ZeroMod8 {}

pub trait AddMod8<Rhs = Self> {
    type Output: Modulo;
}

// identity
impl<T: Modulo> AddMod8<ZeroMod8> for T {
    type Output = T;
}

impl AddMod8<OneMod8> for ZeroMod8 {
    type Output = OneMod8;
}

impl AddMod8<OneMod8> for OneMod8 {
    type Output = TwoMod8;
}

impl AddMod8<OneMod8> for TwoMod8 {
    type Output = ThreeMod8;
}

impl AddMod8<OneMod8> for ThreeMod8 {
    type Output = FourMod8;
}

impl AddMod8<OneMod8> for FourMod8 {
    type Output = FiveMod8;
}

impl AddMod8<OneMod8> for FiveMod8 {
    type Output = SixMod8;
}

impl AddMod8<OneMod8> for SixMod8 {
    type Output = SevenMod8;
}

impl AddMod8<OneMod8> for SevenMod8 {
    type Output = ZeroMod8;
}

// this doesn't work due to infinite recursion
// impl<T, U> AddMod8<U> for T
// where
//     T: AddMod8<OneMod8>,
//     <T as AddMod8<OneMod8>>::Output: AddMod8<OneMod8>,
// {
//     type Output = <<T as AddMod8<OneMod8>>::Output as AddMod8<OneMod8>>::Output;
// }

impl<T> AddMod8<TwoMod8> for T
where
    T: AddMod8<OneMod8>,
    <T as AddMod8<OneMod8>>::Output: AddMod8<OneMod8>,
{
    type Output = <<T as AddMod8<OneMod8>>::Output as AddMod8<OneMod8>>::Output;
}

impl<T> AddMod8<ThreeMod8> for T
where
    T: AddMod8<TwoMod8>,
    <T as AddMod8<TwoMod8>>::Output: AddMod8<OneMod8>,
{
    type Output = <<T as AddMod8<TwoMod8>>::Output as AddMod8<OneMod8>>::Output;
}

impl<T> AddMod8<FourMod8> for T
where
    T: AddMod8<ThreeMod8>,
    <T as AddMod8<ThreeMod8>>::Output: AddMod8<OneMod8>,
{
    type Output = <<T as AddMod8<ThreeMod8>>::Output as AddMod8<OneMod8>>::Output;
}

impl<T> AddMod8<FiveMod8> for T
where
    T: AddMod8<FourMod8>,
    <T as AddMod8<FourMod8>>::Output: AddMod8<OneMod8>,
{
    type Output = <<T as AddMod8<FourMod8>>::Output as AddMod8<OneMod8>>::Output;
}

impl<T> AddMod8<SixMod8> for T
where
    T: AddMod8<FiveMod8>,
    <T as AddMod8<FiveMod8>>::Output: AddMod8<OneMod8>,
{
    type Output = <<T as AddMod8<FiveMod8>>::Output as AddMod8<OneMod8>>::Output;
}

impl<T> AddMod8<SevenMod8> for T
where
    T: AddMod8<SixMod8>,
    <T as AddMod8<SixMod8>>::Output: AddMod8<OneMod8>,
{
    type Output = <<T as AddMod8<SixMod8>>::Output as AddMod8<OneMod8>>::Output;
}

pub type InRange<T> = <<T as TrueFalseArray>::Marker as DiscriminantInRange>::Check;

pub enum True {}
pub enum False {}

pub trait DiscriminantInRange {
    type Check;
}

impl DiscriminantInRange for True {
    type Check = ();
}

pub trait TrueFalseArray {
    type Marker;
}

impl TrueFalseArray for [(); 1] {
    type Marker = True;
}

impl TrueFalseArray for [(); 0] {
    type Marker = False;
}
