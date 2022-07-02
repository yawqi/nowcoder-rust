use std::vec;

fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s).unwrap();
    let n = s.trim().parse::<usize>().unwrap();
    if n <= 2 {
        println!("-1");
        return;
    }
    let results = vec![3,2,4,2];
    println!("{}", results[n%4]);
}