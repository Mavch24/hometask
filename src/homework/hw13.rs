#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    // Функція для обчислення площі прямокутника
    fn area(&self) -> i32 {
        (self.b.x - self.a.x).abs() * (self.a.y - self.b.y).abs()
    }

    // Функція для обчислення площі перетину двох прямокутників
    fn overlap_area(&self, other: &Rectangle) -> i32 {
        let x_overlap = (self.b.x.min(other.b.x) - self.a.x.max(other.a.x)).max(0);
        let y_overlap = (self.a.y.min(other.a.y) - self.b.y.max(other.b.y)).max(0);
        x_overlap * y_overlap
    }
}

// Функція для обчислення загальної зайнятої площі
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;
    let mut overlaps = vec![];

    for i in 0..xs.len() {
        let r1 = &xs[i];
        total_area += r1.area();

        // Перевіряємо на перекриття з іншими прямокутниками
        for j in i + 1..xs.len() {
            let r2 = &xs[j];
            let overlap = r1.overlap_area(r2);
            if overlap > 0 {
                overlaps.push(overlap);
            }
        }
    }

    // Вираховуємо площу перекриттів
    let total_overlap: i32 = overlaps.iter().sum();

    // Підсумкова площа = загальна площа прямокутників - площа перекриттів
    total_area - total_overlap
}

// Тестові дані
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

// Тест для перевірки правильності функції
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_occupied_test() {
        let data = test_data();
        let occupied = area_occupied(&data);
        assert_eq!(occupied, 60);
    }
}

fn main() {
    // Викликаємо функцію area_occupied для тестових даних
    let data = test_data();
    let occupied_area = area_occupied(&data);
    println!("Загальна зайнята площа: {}", occupied_area);
}
