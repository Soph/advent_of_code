fn main() {
    let start = 125730;
    let stop = 579381;

    let mut count = 0;
    for i in start..stop {
        if validate_password(&i.to_string()) {
            count += 1;
        }
    }

    println!("Result1: {}", count);

    let mut count = 0;
    for i in start..stop {
        if validate_password2(&i.to_string()) {
            count += 1;
        }
    }

    println!("Result2: {}", count);
}

fn validate_password(password: &String) -> bool {
    if password.len() != 6 {
        return false;
    }
    let mut last_digit = 0;
    let mut double_digit = false;
    for digit in password.chars() {
        let number = digit.to_digit(10).unwrap();
        if number < last_digit {
            return false;
        }
        if number == last_digit {
            double_digit = true;
        }
        last_digit = number;
    }
    if !double_digit {
        return false;
    }
    true
}

fn validate_password2(password: &String) -> bool {
    if password.len() != 6 {
        return false;
    }
    let mut last_digit = 0;
    let mut double_digit = false;
    let mut same_digit_count = 1;
    for digit in password.chars() {
        let number = digit.to_digit(10).unwrap();
        if number < last_digit {
            return false;
        }
        if number == last_digit {
            same_digit_count += 1;
        } else {
            if same_digit_count == 2 {
                double_digit = true;
            }
            same_digit_count = 1;
        }
        last_digit = number;
    }
    if same_digit_count == 2 {
        return true;
    }
    if !double_digit {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_password() {
        assert_eq!(validate_password(&"111111".to_string()), true);
        assert_eq!(validate_password(&"223450".to_string()), false);
        assert_eq!(validate_password(&"123789".to_string()), false);
    }

    #[test]
    fn test_validate_password2() {
        assert_eq!(validate_password2(&"112233".to_string()), true);
        assert_eq!(validate_password2(&"123444".to_string()), false);
        assert_eq!(validate_password2(&"111122".to_string()), true);
    }
}
