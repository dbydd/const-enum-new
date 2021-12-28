#![feature(const_trait_impl)]

use const_enum::ConstEnum;

#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(u8)]
enum Test {
    A = 0b010,
    B = 0b100,
    C = 0b001
}

#[test]
pub fn test_example() {
    println!("{:?}", Test::from(0b010 as u8));
    println!("{:?}", u8::from(Test::A));
}
