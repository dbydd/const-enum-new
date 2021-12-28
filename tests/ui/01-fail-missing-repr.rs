#![feature(const_trait_impl)]

use const_enum::ConstEnum;

#[derive(ConstEnum)]
enum Test {
    A = 1,
    B = 2,
    C = 3,
}

pub fn main() {}
