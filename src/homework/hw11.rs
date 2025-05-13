use rand::Rng;

pub fn run() {
    // Генерація випадкового вектора довжиною 20
    let mut rng = rand::rngs::ThreadRng::default();  // Використовуємо новий метод
    let data: Vec<i32> = (0..20).map(|_| rng.random_range(10..100)).collect();  // Використовуємо random_range

    // Виведення індексів
    println!(
        "indexes: {}",
        (0..20)
            .map(|i| format!("{:>2}.", i))
            .collect::<Vec<_>>()
            .join("  ")
    );
    
    // Виведення даних
    println!("data:   {:?}", data);

    // Якщо вектор менший за два елементи, виводимо повідомлення і завершуємо функцію
    if data.len() < 2 {
        return;
    }

    // Ініціалізація змінних для мінімальної суми та індексу
    let (mut min_sum, mut min_index) = (i32::MAX, 0);

    // Пошук мінімальної суми сусідніх елементів
    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    // Виведення індексів і підкреслення мінімальної пари
    println!(
        "indexes: {}\\__ __/{}",
        " ".repeat(min_index * 4),
        " ".repeat((data.len() - min_index - 2) * 4)
    );

    // Виведення мінімальної пари сум
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[min_index],
        data[min_index + 1],
        min_sum,
        min_index,
        min_index + 1
    );
}
