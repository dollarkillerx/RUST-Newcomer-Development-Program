#![allow(overflowing_literals)]

// 不显示类型转换产生的溢出警告
use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;

type NewU64 = u64;  // 别名

fn main() {
    test1();
    test2();
}

fn test1() {
    let decimal = 65.4321_f32; // f32

    // let i:u8 = decimal; //  不提供隐式转换

    let _i = decimal as u8;
}

#[derive(Debug)]
struct Number<T> {
    value: T,
}

impl<T> From<T> for Number<T> {
    fn from(item: T) -> Self {
        Number { value: item }
    }
}

fn test2() {
    let my_str = "hello";
    let my_string = String::from(my_str); // &str to String

    println!("my string: {}", my_string);

    let m1 = Number::from(12);
    println!("{:?}", m1);

    let i = 12;
    let m2: Number<i32> = i.into();
    println!("{:?}", m2);
}

#[derive(Debug, PartialOrd)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

