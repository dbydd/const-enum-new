#![feature(const_trait_impl)]
#![feature(repr128)]

use const_enum::ConstEnum;

#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(u128)]
enum TestU128 {
    A = 0b010,
    B = 0b100,
    C = 0b001,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(i128)]
enum TestI128 {
    A = 0b010,
    B = 0b100,
    C = 0b001,
}

pub fn main() {}
