use std::ops::Add;

pub trait Weight: Add<Output = Self> + Copy + Default {
    const ONE: Self;
}

macro_rules! impl_int {
    ($t:ty) => {
        impl Weight for $t {
            const ONE: Self = 1;
        }
    };
}

macro_rules! impl_float {
    ($t:ty) => {
        impl Weight for $t {
            const ONE: Self = 1.0;
        }
    };
}

impl_int!(i8);
impl_int!(i16);
impl_int!(i32);
impl_int!(i64);
impl_int!(i128);
impl_int!(isize);
impl_int!(u8);
impl_int!(u16);
impl_int!(u32);
impl_int!(u64);
impl_int!(u128);
impl_int!(usize);

impl_float!(f32);
impl_float!(f64);
