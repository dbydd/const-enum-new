#![feature(const_trait_impl)]

use const_enum::ConstEnum;

#[derive(ConstEnum)]
struct TestStruct {}

#[derive(ConstEnum)]
union TestUnion {}

pub fn main() {}
