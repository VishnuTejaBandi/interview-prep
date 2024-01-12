use super::*;

#[test]
fn test_prime_factors() {
    assert_eq!(vec![13], prime_factors(13));
    assert_eq!(vec![2, 2, 5], prime_factors(20));
    assert_eq!(vec![2, 3, 5, 7], prime_factors(210));
}

#[test]
fn test_all_divisors() {
    assert_eq!(vec![1, 3, 5, 9, 15, 25, 45, 75, 225], all_divisors(225));
    assert_eq!(vec![1, 3, 5, 15], all_divisors(15));
}

#[test]
fn test_all_primes_below() {
    assert_eq!(vec![] as Vec<i32>, all_primes_below(-1));
    assert_eq!(vec![2], all_primes_below(2));
    assert_eq!(vec![2, 3, 5, 7, 11, 13, 17, 19, 23], all_primes_below(23));
}

#[test]
fn test_power() {
    assert_eq!(5_i32.pow(5), power(5, 5));
    assert_eq!(5_i32.pow(4), power(5, 4));
}

#[test]
fn test_power_iter() {
    assert_eq!(5_i32.pow(5), power_iter(5, 5));
    assert_eq!(5_i32.pow(4), power_iter(5, 4));
}

#[test]
fn test_digits_in_factorial() {
    assert_eq!(3, digits_in_factorial(5));
    assert_eq!(199, digits_in_factorial(120));
}
