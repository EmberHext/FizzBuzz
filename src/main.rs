use std::ops::Range;

mod enums;
use enums::fizz_buzz_result::FizzBuzzResult::*;

fn write_range(r: Range<usize>) -> std::io::Result<()> {
    for i in r {
        match (i % 3, i % 5) {
            (0, 0) => println!("{}", FizzBuzz::<String>),
            (0, _) => println!("{}", Fizz::<String>),
            (_, 0) => println!("{}", Buzz::<String>),
            (_, _) => println!("{}", Number(i))
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    write_range(1..101)
}