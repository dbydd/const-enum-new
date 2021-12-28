#![feature(const_trait_impl)]

use const_enum::ConstEnum;

#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(u8)]
enum TestU8 {
    A = 0b010,
    B = 0b100,
    C = 0b001,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(u16)]
enum TestU16 {
    A = 0b010,
    B = 0b100,
    C = 0b001,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(u32)]
enum TestU32 {
    A = 0b010,
    B = 0b100,
    C = 0b001,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(u64)]
enum TestU64 {
    A = 0b010,
    B = 0b100,
    C = 0b001,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(usize)]
enum TestUsize {
    A = 0b010,
    B = 0b100,
    C = 0b001,
}

pub fn main() {}
