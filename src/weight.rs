use std::ops::Add;

pub trait Weight: Add<Output = Self> + Copy + Default {}

impl Weight for i8 {}
impl Weight for i16 {}
impl Weight for i32 {}
impl Weight for i64 {}
impl Weight for i128 {}
impl Weight for isize {}

impl Weight for u8 {}
impl Weight for u16 {}
impl Weight for u32 {}
impl Weight for u64 {}
impl Weight for u128 {}
impl Weight for usize {}

impl Weight for f32 {}
impl Weight for f64 {}
