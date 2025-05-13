const WIDTH: usize = 9;
const HEIGHT: usize = 5;

pub fn draw_envelope() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // Діагональ зліва направо
            let main_diag = x == y * WIDTH / HEIGHT;
            // Діагональ справа наліво
            let anti_diag = x == (WIDTH - 1) - y * WIDTH / HEIGHT;
            // Рамка по краях
            let border = x == 0 || x == WIDTH - 1 || y == 0 || y == HEIGHT - 1;

            if border || main_diag || anti_diag {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output); // Використовуємо один раз
}

fn main() {
    draw_envelope();
}
