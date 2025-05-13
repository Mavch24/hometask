pub fn calculate_balance_steps(variable_list: &Vec<u32>) -> isize {
    let total_value: u32 = variable_list.iter().sum();
    let count = variable_list.len() as u32;

    // Перевірка, чи можна рівномірно розподілити вантаж
    if total_value % count != 0 {
        return -1;  // Якщо не можна рівномірно розподілити вантаж
    }

    let average = total_value / count;
    let mut step_counter = 0;
    let mut imbalance = 0;

    // Рахуємо кількість кроків для досягнення балансу
    for &variable_one in variable_list {
        imbalance += variable_one as i32 - average as i32;
        step_counter += imbalance.abs() as usize;
    }

    step_counter as isize
}

pub fn generate_balanced_vector(size: usize) -> Vec<u32> {
    // Оновлений код для генерації випадкових значень
    let average_value = rand::random::<u32>() % 100 + 1;  // Генерація випадкового числа від 1 до 100
    let mut generated_vector = vec![average_value; size];

    // Генерація вектора з варіаціями для рівномірного розподілу
    for index in 0..size / 2 {
        let variation = rand::random::<u32>() % (average_value.min(10) + 1);  // Генерація випадкової варіації
        generated_vector[index] += variation;
        generated_vector[size - 1 - index] -= variation;
    }

    generated_vector
}

pub fn main() {
    let example_one = vec![8, 2, 2, 4, 4];
    let result_one = calculate_balance_steps(&example_one);
    println!("Example 1: {:?} => Moves: {}", example_one, result_one);

    let example_two = vec![9, 3, 7, 2, 9];
    let result_two = calculate_balance_steps(&example_two);
    println!("Example 2: {:?} => Moves: {}", example_two, result_two);

    let example_three = vec![1, 1, 1, 1, 6];
    let result_three = calculate_balance_steps(&example_three);
    println!("Example 3: {:?} => Moves: {}", example_three, result_three);

    let generated = generate_balanced_vector(10);
    println!("Generated vector: {:?}", generated);
    let generated_result = calculate_balance_steps(&generated);
    println!("Moves required to balance generated vector: {}", generated_result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_one() {
        let test_data = vec![8, 2, 2, 4, 4];
        assert_eq!(calculate_balance_steps(&test_data), 6);
    }

    #[test]
    fn test_case_two() {
        let test_data = vec![9, 3, 7, 2, 9];
        assert_eq!(calculate_balance_steps(&test_data), 7);
    }
}
