fn main() {
    const LEVELS: usize = 5;
    let mut tree = String::new();

    let max_width = 2 * LEVELS + 1; // максимальна ширина, визначена за кількістю рівнів

    // Генерація кожного рівня ялинки
    for level in 1..=LEVELS {
        for row in 1..=level {
            let stars = "*".repeat(2 * row - 1); // кількість зірочок
            let spaces = " ".repeat((max_width - stars.len()) / 2); // відступи для вирівнювання
            tree.push_str(&format!("{}{}\n", spaces, stars)); // додаємо рядок до дерева
        }
    }

    // Виводимо результат
    print!("{}", tree);
}
