use std::ops::Range;
use num_traits::PrimInt;

mod enums;
use enums::fizz_buzz_result::FizzBuzzResult::{*, self};

fn fizz_type<T>(num: T) -> FizzBuzzResult<T>
where
T: PrimInt {
    let zero = T::zero();
    let three = T::from(3).expect("Failure converting 3");
    let five = T::from(5).expect("Failure converting 5");
    match (num % three, num % five) {
        (a, b) if a == zero && b == zero => FizzBuzz,
        (a, _) if a == zero => Fizz,
        (_, a) if a == zero => Buzz,
        (_, _) => Number(num)
    }
}

fn write_range(r: Range<usize>) -> std::io::Result<()> {
    for i in r {
        println!("{}", fizz_type(i));
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    write_range(1..101)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz() {
        assert_eq!(fizz_type(3), Fizz);
        assert_eq!(fizz_type(9), Fizz);
        assert_eq!(fizz_type(33), Fizz);
    }
    
    #[test]
    fn test_buzz() {
        assert_eq!(fizz_type(5), Buzz);
        assert_eq!(fizz_type(20), Buzz);
        assert_eq!(fizz_type(50), Buzz);
    }

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(fizz_type(15), FizzBuzz);
        assert_eq!(fizz_type(45), FizzBuzz);
        assert_eq!(fizz_type(60), FizzBuzz);
    }
}