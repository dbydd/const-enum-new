#![feature(const_trait_impl)]

use const_enum::ConstEnum;

#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(i8)]
enum TestI8 {
    A = 0b010,
    B = 0b100,
    C = 0b001,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(i16)]
enum TestI16 {
    A = 0b010,
    B = 0b100,
    C = 0b001,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(i32)]
enum TestI32 {
    A = 0b010,
    B = 0b100,
    C = 0b001,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(i64)]
enum TestI64 {
    A = 0b010,
    B = 0b100,
    C = 0b001,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(isize)]
enum TestIsize {
    A = 0b010,
    B = 0b100,
    C = 0b001,
}

pub fn main() {}
