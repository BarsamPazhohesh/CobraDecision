pub fn my_atoi(s: String) -> i32 {
    let mut num: i32 = 0;          // Final number result
    let mut sign: i32 = 1;         // Sign multiplier (1 or -1)
    let mut started: bool = false; // Indicates if parsing of digits or sign has started

    for ch in s.chars() {
        // Skip leading whitespace characters
        if !started && ch.is_whitespace() {
            continue;
        }

        // Handle optional '+' or '-' sign (only before number starts)
        if !started && (ch == '+' || ch == '-') {
            sign = if ch == '-' { -1 } else { 1 };
            started = true;
            continue;
        }

        // If the character is a digit, process it
        if ch.is_ascii_digit() {
            // Convert char to digit value ('0' → 0, '1' → 1, ..., '9' → 9)
            let digit = ch as i32 - '0' as i32;

            // Safely multiply existing number by 10 and add new digit
            if let Some(res) = num.checked_mul(10).and_then(|res| res.checked_add(digit)) {
                num = res;
            } else {
                // Overflow: return clamped value depending on sign
                return if sign == 1 { i32::MAX } else { i32::MIN };
            }

            started = true; // Mark that we've started parsing the number
        } else {
            // Non-digit character found after number started: stop parsing
            break;
        }
    }

    num * sign // Apply sign and return final result
}

#[cfg(test)]
mod string_to_integer_atoi {
    use super::*;

    #[test]
    fn test_positive_number() {
        assert_eq!(my_atoi("42".to_string()), 42);
    }

    #[test]
    fn test_negative_number() {
        assert_eq!(my_atoi("-42".to_string()), -42);
    }

    #[test]
    fn test_with_leading_whitespace() {
        assert_eq!(my_atoi("   123".to_string()), 123);
    }

    #[test]
    fn test_with_plus_sign() {
        assert_eq!(my_atoi("+987".to_string()), 987);
    }

    #[test]
    fn test_with_non_digit_trailing_chars() {
        assert_eq!(my_atoi("123abc".to_string()), 123);
    }

    #[test]
    fn test_overflow_positive() {
        assert_eq!(my_atoi("91283472332".to_string()), i32::MAX);
    }

    #[test]
    fn test_overflow_negative() {
        assert_eq!(my_atoi("-91283472332".to_string()), i32::MIN);
    }

    #[test]
    fn test_only_sign() {
        assert_eq!(my_atoi("-".to_string()), 0);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(my_atoi("".to_string()), 0);
    }

    #[test]
    fn test_mixed_sign_and_whitespace() {
        assert_eq!(my_atoi("   -0012a42".to_string()), -12);
    }
}
