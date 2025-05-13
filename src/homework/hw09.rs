pub fn rotate2(s: String, n: isize) -> String {
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

#[cfg(test)]
mod tests {
    use super::rotate2;

    #[test]
    fn test_rotate2() {
        let data = [
            ("abcdefgh", 0, "abcdefgh"),
            ("abcdefgh", 8, "abcdefgh"),
            ("abcdefgh", -8, "abcdefgh"),
            ("abcdefgh", 1, "habcdefg"),
            ("abcdefgh", 2, "ghabcdef"),
            ("abcdefgh", 10, "ghabcdef"),
            ("abcdefgh", -1, "bcdefgha"),
            ("abcdefgh", -2, "cdefghab"),
            ("abcdefgh", -10, "cdefghab"),
        ];

        for &(input, shift, expected) in &data {
            assert_eq!(rotate2(input.to_string(), shift), expected.to_string());
        }
    }
}
