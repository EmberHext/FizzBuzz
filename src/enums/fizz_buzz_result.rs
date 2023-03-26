use std::fmt;

pub enum FizzBuzzResult<T> {
    Fizz,
    Buzz,
    FizzBuzz,
    Number (T)
}

impl<T: fmt::Display> fmt::Display for FizzBuzzResult<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FizzBuzzResult::Fizz => write!(f, "Fizz"),
            FizzBuzzResult::Buzz => write!(f, "Buzz"),
            FizzBuzzResult::FizzBuzz => write!(f, "FizzBuzz"),
            FizzBuzzResult::Number(t) => write!(f, "{}", t)
        }
    }
}