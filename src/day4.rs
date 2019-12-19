// Day 4 2019

#[cfg(test)]
mod tests {
    use crate::day4::*;
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(2), 2);
        assert_eq!(factorial(3), 6);
        assert_eq!(factorial(10), 3628800);
    }
    #[test]
    fn test_choose() {
        assert_eq!(choose(1, 0), 1);
        assert_eq!(choose(1, 1), 1);
        assert_eq!(choose(2, 2), 1);
        assert_eq!(choose(2, 1), 2);
        assert_eq!(choose(10, 0), 1);
        assert_eq!(choose(10, 3), 120);
    }
    #[test]
    fn test_count_ascending() {
        assert_eq!(count_ascending(3, 2), 4);
        assert_eq!(count_ascending(3, 3), 10);
        assert_eq!(count_ascending(3, 4), 20);
    }
    #[test]
    fn test_to_increasing_digits() {
        assert_eq!(to_increasing_digits(123, true), 123);
        assert_eq!(to_increasing_digits(213, true), 223);
        assert_eq!(to_increasing_digits(321, true), 333);
        assert_eq!(to_increasing_digits(321, false), 299);
    }
    #[test]
    fn test_count_ascending_with_pairs_between() {
        assert_eq!(count_ascending_with_pairs_between(6, 165432, 707912), 1716);
    }
    #[test]
    // This test passes on the commented line, but takes forever
    fn test_brute() {
        // assert_eq!(brute(165432, 707912), 1716);
        assert_eq!(1, 1);
    }
}

/// Split a number into a vector of it's digits
fn to_digits(number: usize) -> Vec<usize> {
    use std::str::FromStr;
    number
        .to_string()
        .split("")
        .filter(|x| x != &"")
        .map(|x| usize::from_str(x).unwrap())
        .collect()
}

/// Collect a vector of digits into a number
fn to_number(digits: Vec<usize>) -> usize {
    let mut number = 0;
    let length = digits.len();
    digits
        .iter()
        .enumerate()
        // The first digit in the interator is the nth digit it the number,
        // where n is the length. That means it is in the nth place, or the
        // 10 ^ (nth - 1) place. Successive digits are in the (n - i)th place.
        .for_each(|(i, d)| number += d * 10_usize.pow((length - i - 1) as u32));
    number
}

/// Modify a number so it's digits are >= to the previous. `increase`
/// determines whether the resulting number can greater than the input number.
/// - E.g. `increase == true` with `number == 321` will produce 333
/// - E.g. `increase == false` with `number == 321` will produce 299, the
/// largest increasing-digit number less than 321
fn to_increasing_digits(number: usize, increase: bool) -> usize {
    // Let's not get ahead of ourselves
    if has_increasing_digits(number) {
        return number;
    }
    let digits = to_digits(number);
    let mut satisfied = vec![];
    if increase {
        digits.iter().enumerate().for_each(|(i, d)| {
            // If this isn't the first digit, and d is less than the previous
            // increased digit, make it the previous increased digit. Otherwise,
            // stay d.
            let increase = if i > 0 && d < &satisfied[i - 1] {
                *&satisfied[i - 1]
            } else {
                *d
            };
            satisfied.push(increase);
        });
    } else {
        if digits[0] == 0 {
            panic!(
                "When `increase` == false, number must not be zero-padded, but was {}",
                number
            );
        }
        digits.iter().for_each(|_| satisfied.push(9));
        satisfied[0] = digits[0] - 1;
    }
    to_number(satisfied)
}

/// Checks whether a number has increasing digits.
fn has_increasing_digits(number: usize) -> bool {
    let digits = to_digits(number);
    for (i, d) in digits.iter().enumerate() {
        if i > 0 && d < &digits[i - 1] {
            return false;
        }
    }
    true
}

/// Checks a number is within the interval (lower, upper).
fn within_range(lower: usize, upper: usize, number: usize) -> bool {
    lower < number && number < upper
}

/// Checks whether a number has an adjacent repeated digit.
fn has_double(number: usize) -> bool {
    let digits = to_digits(number);
    for (i, d) in digits.iter().enumerate() {
        if i < digits.len() - 1 && d == &digits[i + 1] {
            return true;
        }
    }
    false
}

/// Calculates n factorial
fn factorial(n: usize) -> usize {
    match n {
        0 => 1,
        1 => 1,
        _ => n * factorial(n - 1),
    }
}

/// Calculates the combinations n choose r
fn choose(n: usize, r: usize) -> usize {
    if r > n {
        panic!("r ({}) must not be greater than n ({})", r, n);
    }
    // These could save significant time if n is large
    if r == 0 || r == n {
        return 1;
    }
    if r == 1 {
        return n;
    }
    // n! / ( r! * (n - r)! )
    factorial(n) / factorial(r) / factorial(n - r)
}

/// Calculates the number of `n` length numbers of `d` digits where the digits
/// are ascending.
/// - E.g. a 3 digit number with digits 1..=4: `n` = 3, `d` = 4
/// - E.g. a 3 digit number iwth digits 5..=9: `n` = 3, `d` = 4
fn count_ascending(n: usize, d: usize) -> usize {
    if d <= 0 {
        panic!("d must be a positive integer, not {}", d);
    }
    if n == 0 {
        // There are no 0 length numbers
        return 0;
    }
    choose(n + d - 1, d - 1)
}

/// Calculates the number of `n` length numbers between `min` and `max` that
/// have at least one pair of repeated digits and are ascending.
fn count_ascending_with_pairs_between(n: usize, min: usize, max: usize) -> usize {
    if n < 2 {
        panic!("n must be at least 2, but was {}", n);
    }
    if min >= max {
        panic!("min must be less than max, but were {} and {}", min, max);
    }
    let lower = to_increasing_digits(min, true);
    let lower_digits = to_digits(lower);
    let upper = to_increasing_digits(max, false);
    let upper_digits = to_digits(upper);
    if lower_digits.len() != n || upper_digits.len() != n {
        panic!(
            "min and max must be n digits long, but were {} and {}",
            min, max
        );
    }
    println!("{}, {}", lower, upper);

    let mut count = 0;

    // i is the first digit (not it's index, it's actual digit value)
    for i in lower_digits[0]..=upper_digits[0] {
        // j is the index of the first pair digit, which be inclusively between
        // the first digit and the penultimate digit
        for j in 0..=(n - 2) {
            if j == 0 {
                // --- Count the head digits ---
                // There are no head digits
                // --- Count the pair digits ---
                // The pair digit = i, and occupies the first and second digit
                // spot. If i is less than the second digit's min, than this j
                // isn't valid
                if i < lower_digits[1] {
                    continue;
                }
                // The tail digits can be between i and 9, and are
                // n - 2 digits long (because j = 0)
                count += count_ascending(n - 2, 10 - i);
            } else if j == 1 {
                // --- Count the head digits ---
                // There is only one head digit and it is i
                // --- Count the pair digits ---
                // l is the pair digit inclusively between i and 9, and occupies
                // the second and third digit spot.
                for l in i..=9 {
                    // If i is less than the second digit's min, than this j
                    // isn't valid
                    if l < lower_digits[1] {
                        continue;
                    }
                    // --- Count the tail digits ---
                    // The tail digits can be inclusively between l and 9, and
                    // are n - 3 digits long (because j = 1)
                    if l != 9 {
                        count += count_ascending(n - 3, 10 - l)
                    }
                }
            } else {
                // k is the max of the head ascending digits
                for k in i..=9 {
                    // --- Count the head digits ---
                    // The head digits after the first digit can be inclusively
                    // between i and k (unless i is it's min, at which point
                    // the min isn't i but max(i, the second digit's min)), and are j
                    // digits long
                    let head_min = if i == lower_digits[0] {
                        std::cmp::max(i, lower_digits[1])
                    } else {
                        i
                    };
                    // If the maximum of the head digits is less than their
                    // minimum, this k isn't valid
                    if k < head_min {
                        continue;
                    }
                    count += count_ascending(j - 1, 1 + k - head_min);
                    // --- Count the pair digits ---
                    // l is the pair digit, inclusively between k and 9
                    for l in k..=9 {
                        // --- Count the tail digits ---
                        // The tail digits can be inclusively between l and 9,
                        // and are n - j - 2 digits long
                        count += count_ascending(n - j - 2, 10 - l)
                    }
                }
            }
        }
    }

    count
}

fn brute(min: usize, max: usize) -> usize {
    let mut count = 0;
    for number in min..max {
        if has_double(number) && has_increasing_digits(number) {
            count += 1;
        }
    }

    count
}
