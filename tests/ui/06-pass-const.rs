#![feature(const_trait_impl)]

use const_enum::ConstEnum;

#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(u8)]
enum Test {
    A = 0b010,
    B = 0b100,
    C = 0b001
}

pub fn main() {
    println!("{:?}", to_enum(0b10));
    println!("{:?}", from_enum(Test::B));
}

const fn to_enum(value: u8) -> Test {
    Test::from(value)
}

const fn from_enum(value: Test) -> u8 {
    u8::from(value)
}
