use bitfield::*;
type A = B1;
type B = B3;
type C = B4;
type D = B23;
pub struct NotQuiteFourBytes {
    data: [u8; (<A as Specifier>::BITS
        + <B as Specifier>::BITS
        + <C as Specifier>::BITS
        + <D as Specifier>::BITS)
        / 8],
}
#[automatically_derived]
impl ::core::default::Default for NotQuiteFourBytes {
    #[inline]
    fn default() -> NotQuiteFourBytes {
        NotQuiteFourBytes {
            data: ::core::default::Default::default(),
        }
    }
}
impl NotQuiteFourBytes {
    pub fn get_a(&self) -> <A as ValueGetSet>::ValueType {
        let offset = 0;
        <A as ValueGetSet>::get(&self.data, offset)
    }
    pub fn set_a(&mut self, value: <A as ValueGetSet>::ValueType) {
        let offset = 0;
        <A as ValueGetSet>::set(&mut self.data, offset, value);
    }
}
impl NotQuiteFourBytes {
    pub fn get_b(&self) -> <B as ValueGetSet>::ValueType {
        let offset = 0 + <A as Specifier>::BITS;
        <B as ValueGetSet>::get(&self.data, offset)
    }
    pub fn set_b(&mut self, value: <B as ValueGetSet>::ValueType) {
        let offset = 0 + <A as Specifier>::BITS;
        <B as ValueGetSet>::set(&mut self.data, offset, value);
    }
}
impl NotQuiteFourBytes {
    pub fn get_c(&self) -> <C as ValueGetSet>::ValueType {
        let offset = 0 + <A as Specifier>::BITS + <B as Specifier>::BITS;
        <C as ValueGetSet>::get(&self.data, offset)
    }
    pub fn set_c(&mut self, value: <C as ValueGetSet>::ValueType) {
        let offset = 0 + <A as Specifier>::BITS + <B as Specifier>::BITS;
        <C as ValueGetSet>::set(&mut self.data, offset, value);
    }
}
impl NotQuiteFourBytes {
    pub fn get_d(&self) -> <D as ValueGetSet>::ValueType {
        let offset = 0 + <A as Specifier>::BITS + <B as Specifier>::BITS + <C as Specifier>::BITS;
        <D as ValueGetSet>::get(&self.data, offset)
    }
    pub fn set_d(&mut self, value: <D as ValueGetSet>::ValueType) {
        let offset = 0 + <A as Specifier>::BITS + <B as Specifier>::BITS + <C as Specifier>::BITS;
        <D as ValueGetSet>::set(&mut self.data, offset, value);
    }
}
impl NotQuiteFourBytes {
    pub fn new() -> Self {
        Default::default()
    }
}
fn _check() {
    use bitfield::checks::*;
    let _: MultipleOfEight<
        [(); <A as Specifier>::BITS
            + <B as Specifier>::BITS
            + <C as Specifier>::BITS
            + <D as Specifier>::BITS % 8],
    > = ();
}
fn main() {}
