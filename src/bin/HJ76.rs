fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n);
    let n = n.trim().parse::<i32>().unwrap();
    let n3 = n * n * n;
    let mut start = n3 / n;
    if n & 1 != 0 {
        start -= 2 * (n / 2);
    } else {
        start -= 1 + 2 * (n / 2 - 1);
    }
    let mut ret = vec![];
    for _ in 0..n {
        ret.push(start.to_string());
        start += 2;
    }

    println!("{}", ret.join("+"));
}