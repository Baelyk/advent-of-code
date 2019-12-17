// Day 4 2019

/// Split a number into a vector of it's digits
fn to_digits(number: usize) -> Vec<usize> {
    use std::str::FromStr;
    number
        .to_string()
        .split("")
        .map(|x| usize::from_str(x).unwrap())
        .collect()
}

/// Modify a number so it's digits are >= to the previous. Increases digits
/// subsequent digits to statisfy the requirement.
fn to_increasing_digits(number: usize) -> usize {
    let digits = to_digits(number);
    let mut satisfied = 0;
    digits.iter().enumerate().for_each(|(i, d)| {
        let increase = if i > 0 && d < &digits[i - 1] {
            *&digits[i - 1]
        } else {
            *d
        };
        satisfied += increase * 10usize.pow((digits.len() - i - 1) as u32);
    });
    satisfied
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
