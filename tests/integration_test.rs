use fizzbuzzlib;
mod tests {
    use fizzbuzzlib::fizz_buzz_result::FizzBuzzResult;

    use super::*;

    #[test]
    fn test_fizz() {
        assert_eq!(fizzbuzzlib::fizz_type(3), FizzBuzzResult::Fizz);
        assert_eq!(fizzbuzzlib::fizz_type(9), FizzBuzzResult::Fizz);
        assert_eq!(fizzbuzzlib::fizz_type(33), FizzBuzzResult::Fizz);
    }
    
    #[test]
    fn test_buzz() {
        assert_eq!(fizzbuzzlib::fizz_type(5), FizzBuzzResult::Buzz);
        assert_eq!(fizzbuzzlib::fizz_type(20), FizzBuzzResult::Buzz);
        assert_eq!(fizzbuzzlib::fizz_type(50), FizzBuzzResult::Buzz);
    }

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(fizzbuzzlib::fizz_type(15), FizzBuzzResult::FizzBuzz);
        assert_eq!(fizzbuzzlib::fizz_type(45), FizzBuzzResult::FizzBuzz);
        assert_eq!(fizzbuzzlib::fizz_type(60), FizzBuzzResult::FizzBuzz);
    }
}