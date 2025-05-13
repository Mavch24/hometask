fn invert_the_case(s: String) -> String {
    let mut result = String::with_capacity(s.len());

    for c in s.chars() {
        if c.is_lowercase() {
            result.push(c.to_uppercase().next().unwrap()); // перетворюємо в верхній регістр
        } else if c.is_uppercase() {
            result.push(c.to_lowercase().next().unwrap()); // перетворюємо в нижній регістр
        } else {
            result.push(c); // залишаємо символ без змін
        }
    }

    result
}

#[test]
fn test_invert_the_case() {
    let data = [
        ("Hello",  "hELLO"),
        ("Привет","пРИВЕТ"),
    ];

    for &(input, expected) in &data {
        assert_eq!(invert_the_case(input.to_string()), expected.to_string());
        assert_eq!(invert_the_case(expected.to_string()), input.to_string());
    }
}
