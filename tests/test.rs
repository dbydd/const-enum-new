#![feature(const_trait_impl)]

use const_enum::ConstEnum;

#[derive(Copy, Clone, Debug, Eq, PartialEq, ConstEnum)]
#[repr(u8)]
enum Test {
    A = 0b010,
    B = 0b100,
    C = 0b001,
}

#[test]
pub fn test_enum_to_u8() {
    assert_eq!(Test::from(0b010), Test::A);
    assert_eq!(Test::from(0b100), Test::B);
    assert_eq!(Test::from(0b001), Test::C);
}

#[test]
pub fn test_u8_to_enum() {
    assert_eq!(u8::from(Test::A), 0b010);
    assert_eq!(u8::from(Test::B), 0b100);
    assert_eq!(u8::from(Test::C), 0b001);
}

#[test]
#[should_panic]
pub fn test_invalid_variant() {
    Test::from(0b111);
}
