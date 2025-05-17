pub fn solve_crypto_task() {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut count = 0;

    for &m in &digits {
        for &u in &digits {
            if u == m { continue; }
            for &x in &digits {
                if x == m || x == u { continue; }
                for &a in &digits {
                    if [m, u, x].contains(&a) { continue; }

                    let muha = 1000 * m + 100 * u + 10 * x + a;
                    let multiplier = a;
                    let product = muha * multiplier;

                    if product > 9999 || product < 1000 {
                        continue;
                    }

                    let s = (product / 1000) % 10;
                    let l = (product / 100) % 10;
                    let o = (product / 10) % 10;
                    let n = product % 10;

                    let all = [m, u, x, a, s, l, o, n];
                    let mut uniq = all.to_vec();
                    uniq.sort();
                    uniq.dedup();

                    if uniq.len() == 8 && uniq.iter().all(|&d| d >= 1 && d <= 8) {
                        count += 1;
                        println!("  {}{}{}{}", m, u, x, a);
                        println!("x     {}", a);
                        println!("------");
                        println!("  {}{}{}{}", s, l, o, n);
                        println!();
                    }
                }
            }
        }
    }

    println!("Загальна кількість рішень: {}", count);
}
