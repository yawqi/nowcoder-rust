fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();
    let mut count = 0;
    for _ in 1..=n {
        if n % 7 == 0 {
            count += 1;
            continue;
        }
        if n % 10 == 7
        || (n % 100 >= 70 && n % 100 < 80)
        || (n % 1000 >= 700 && n % 1000 < 800)
        || (n % 10000 >= 7000 && n % 10000 < 8000) {
            count += 1;
        }
    }
    println!("{}", count);
}