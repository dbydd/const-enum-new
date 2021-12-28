#![feature(const_trait_impl)]

use const_enum::ConstEnum;

#[derive(ConstEnum)]
#[repr(u8, u8)]
enum Test {
    A = 1,
    B = 2,
    C = 3,
}

pub fn main() {}
