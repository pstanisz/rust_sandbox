// Copyright (c) 2023 Piotr Staniszewski

const MAX_N_DIGIT_NUMBER: u64 = 999;
const MAX_PRODUCT_OF_N_DIGIT_NUMBER: u64 = MAX_N_DIGIT_NUMBER * MAX_N_DIGIT_NUMBER;


fn is_palindrome(number: u64) -> bool {
    let mut working_number = number;

    let mut rev_number: u64 = 0;
    while working_number > 0 {
        rev_number = rev_number * 10 + working_number % 10;
        working_number /= 10;
    }
    
    rev_number == number
}

fn first_idea() -> (u64, u64, u64)
{
    let mut largest_palindrome: u64 = 0;
    let mut number1: u64 = 0;
    let mut number2: u64 = 0;
    
    let mut n = MAX_PRODUCT_OF_N_DIGIT_NUMBER;
    'top: while n > 0 {
        if is_palindrome(n) {
            let mut num1 = MAX_N_DIGIT_NUMBER;
            while num1 > 0 {
                let num2 = n / num1;
                if num2 > MAX_N_DIGIT_NUMBER {
                    break;
                }

                if n % num1 == 0 {
                    largest_palindrome = n;
                    number1 = num1;
                    number2 = num2;

                    break 'top;
                }

                num1 = num1 - 1;
            }
        }
        n = n - 1;
    }

    (largest_palindrome, number1, number2)
}

fn main() {
    println!("Calculates the largest palindrome which is a product of two 3-digits numbers");

    let value1 = 10000;
    println!("is {} palindrome? {}", value1, is_palindrome(value1));

    let value2 = 10001;
    println!("is {} palindrome? {}", value2, is_palindrome(value2));

    let result = first_idea();
    println!("largest palindrome of two 3-digits numbers: {} = {} x {}", result.0, result.1, result.2);
}

#[cfg(test)]
mod tests {
    use crate::is_palindrome;

    #[test]
    fn is_palindrome_test() {
        assert_eq!(is_palindrome(1), true);
        assert_eq!(is_palindrome(10), false);
        assert_eq!(is_palindrome(101), true);
        assert_eq!(is_palindrome(9881), false);
        assert_eq!(is_palindrome(9889), true);
        assert_eq!(is_palindrome(23000032), true);
    }
}