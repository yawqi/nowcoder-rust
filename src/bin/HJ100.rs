fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n);
    let n = n.trim().parse::<u64>().unwrap();

    let sum = 2 * n + n * (n - 1) * 3 / 2;
    println!("{}", sum);
}