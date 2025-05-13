// Функція для обчислення найбільшого спільного дільника (GCD)
fn gcd(a: i32, b: i32) -> i32 {
    let mut x = a;
    let mut y = b;

    while y != 0 {
        let temp = y;
        y = x % y; // Залишок від ділення
        x = temp;
    }

    x.abs() // Повертаємо абсолютне значення GCD
}

fn main() {
    let num1 = 56;
    let num2 = 98;

    println!("GCD of {} and {} is {}", num1, num2, gcd(num1, num2));
}
