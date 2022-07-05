fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n).unwrap();
    for x in 0..20 {
        let mut sum = 5.0 * x as f64;
        for y in 0..34 {
            sum += 3.0 * y as f64 + (100.0 - x as f64 - y as f64) / 3.0;
            if sum == 100.0 {
                println!("{} {} {}", x, y, 100-x-y);
            }
            sum -= 3.0 * y as f64 + (100.0 - x as f64 - y as f64) / 3.0;
        }
    }
}