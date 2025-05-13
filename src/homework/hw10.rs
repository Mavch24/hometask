pub fn rotate(s: String, n: isize) -> String {
    let len = s.chars().count();
    if len == 0 {
        return s;
    }

    let shift = (n.rem_euclid(len as isize)) as usize;
    if shift == 0 {
        return s;
    }

    let chars: Vec<char> = s.chars().collect();
    let split_at = len - shift;

    let mut rotated = String::with_capacity(s.len());
    rotated.extend(chars[split_at..].iter());
    rotated.extend(chars[..split_at].iter());
    rotated
}

pub fn is_palindrome(x: u32) -> bool {
    let original = x;
    let mut n = x;
    let mut rev: u32 = 0;

    while n > 0 {
        let digit = n % 10;
        rev = match rev
            .checked_mul(10)
            .and_then(|v| v.checked_add(digit))
        {
            Some(v) => v,
            None => return false, // Якщо переповнення, повертаємо false
        };
        n /= 10;
    }

    rev == original
}

#[cfg(test)]
mod tests {
    use super::{rotate, is_palindrome};

    #[test]
    fn test_rotate() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate(s.clone(), *n), exp.to_string());
        });
    }

    #[test]
    fn test_palindromes() {
        let test_data = [
            (123, false),
            (121, true),
            (1221, true),
            (12321, true),
            (1001, true),
            (12345, false),
        ];

        test_data.iter().for_each(|(n, expected)| {
            assert_eq!(is_palindrome(*n), *expected);
        });
    }
}
