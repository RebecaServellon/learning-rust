// Conversion
#![allow(dead_code)]

// Defining a conversion
use std::convert::From;
// use std::convert::Into;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number {value: item}
    }
}

/* 
// The into trait is implemented for any type that implements From
impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self}
    }
}
*/

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        }
        else {
            Err(())
        }
    }
}

fn main() {
    // From
    // Allows for a type to define how to create itself from another type
    let _my_str = "hello";
    let _my_string = String::from(_my_str);

    // Using a pre defined conversion
    let num = Number::from(19);
    println!("My number is {:?}", num);

    // ___________________________________

    // Into
    // Defines how to convert a type into another type
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);

    // ____________________________________
    // From and Into are interchangable
    // Both are designed to be complementary, but if the From trait is implemented, so is Into, but the converse is not true.

    // ____________________________________

    // TryFrom and TryInto, similar to From and Into but designed for fallible conversions, return a Result type

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));

    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    // _____________________________________

    // To and from Strings
    // Rather than only implement ToString methos is better implement fmt::Display trait which automatically provides ToString
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // _____________________________________

    // Parsing a string
    // Parse function for converting string to number using the turbofish syntax
    // This will convert the string into the type specified as long the FromStr trait is implemented for that type
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum {:?}", sum);

}
