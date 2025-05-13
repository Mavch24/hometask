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

#[cfg(test)]
mod tests {
    use super::rotate;

    #[test]
    fn test() {
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
}
