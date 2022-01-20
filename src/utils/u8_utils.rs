pub trait WrappingAdd {
    /// Add a signed value to a number, wrapping if it overflowed..
    fn w_add(self, rhs: isize) -> Self;
}

impl WrappingAdd for u8 {
    fn w_add(self, rhs: isize) -> u8 {
        self.wrapping_add(isize_to_u8(rhs))
    }
}

fn isize_to_u8(i: isize) -> u8 {
    (i & u8::MAX as isize) as u8
}
