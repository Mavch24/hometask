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
    use super::is_palindrome;

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
