fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s).unwrap();

    let n = s.trim().parse::<usize>().unwrap();
    let mut v = [1; 32];
    for i in 2..n {
        v[i] = v[i-1] + v[i-2];
    }
    println!("{}", v[n-1]);
}