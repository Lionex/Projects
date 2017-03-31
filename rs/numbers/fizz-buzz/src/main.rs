#![feature(type_ascription)]
use std::fmt;

#[derive(Debug,PartialEq,Eq)]
enum FizzBuzz<T> {
    Value(T),
    Fizz(T),
    Buzz(T),
    FizzBuzz(T),
}

impl From<u64> for FizzBuzz<u64> {
    fn from(n: u64) -> Self {
        use FizzBuzz::*;
        match n {
            n if n % 15 == 0 => FizzBuzz(n),
            n if n % 5  == 0 => Buzz(n),
            n if n % 3  == 0 => Fizz(n),
            _                => Value(n),
        }
    }
}

impl<T: fmt::Display> fmt::Display for FizzBuzz<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &FizzBuzz::FizzBuzz(_)  => write!(f, "FizzBuzz"),
            &FizzBuzz::Buzz(_)      => write!(f, "Buzz"),
            &FizzBuzz::Fizz(_)      => write!(f, "Fizz"),
            &FizzBuzz::Value(ref x) => write!(f, "{}", x),
        }
    }
}

fn main() {
    for fizz_buzz in (1..101).map(|x| FizzBuzz::from(x)) {
        println!("{}", fizz_buzz);
    }
}

#[test]
fn fizzbuzz_from_u64() {
    assert_eq!(FizzBuzz::FizzBuzz(15),FizzBuzz::from(15));
    assert_eq!(FizzBuzz::Fizz(3),FizzBuzz::from(3));
    assert_eq!(FizzBuzz::Buzz(10),FizzBuzz::from(10));
    assert_eq!(FizzBuzz::Value(1),FizzBuzz::from(1));
}

#[test]
#[should_panic]
fn invalid_fizzbuzz() { assert_eq!(FizzBuzz::Value(9u64),FizzBuzz::from(9)); }
