fn is_prime(n: &u32) -> bool {
    if *n < 2 {
        return false; // Числа менше 2 не є простими
    }
    let sqrt_n = (*n as f64).sqrt() as u32; // Обчислюємо квадратний корінь числа
    for i in 2..=sqrt_n {
        if *n % i == 0 {
            return false; // Якщо знайдений дільник, число не є простим
        }
    }
    true // Якщо не знайдено дільників, число є простим
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let test_data = [
            (0, false),
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (100, false),
            (10007, true),
        ];

        test_data.iter().for_each(|(n, prime)| {
            assert_eq!(is_prime(n), *prime); // Перевірка для кожного числа з тестового масиву
        });
    }
}
