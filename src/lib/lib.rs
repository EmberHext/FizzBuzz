use num_traits::PrimInt;
pub mod fizz_buzz_result;
use fizz_buzz_result::FizzBuzzResult::{*, self};
use std::{ops::Range, io::Write};

pub fn fizz_type<T>(num: T) -> FizzBuzzResult<T>
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

pub fn write_range<W: Write>(mut w: W, r: Range<usize>) -> std::io::Result<()> {
    for i in r {
        writeln!(w, "{}", fizz_type(i))?;
    }
    Ok(())
}
