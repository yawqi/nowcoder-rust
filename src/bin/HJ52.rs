fn main() {
    let mut a = String::new();
    let mut b = String::new();
    std::io::stdin()
        .read_line(&mut a).unwrap();
    std::io::stdin()
        .read_line(&mut b).unwrap();
    let a = a.trim().chars().collect::<Vec<_>>();
    let b = b.trim().chars().collect::<Vec<_>>();

    let mut prev = (0..).step_by(1).take(b.len()+1).collect::<Vec<_>>();
    let mut curr;
    for i in 1..=a.len() {
        curr = vec![i; b.len()+1];
        for j in 1..=b.len() {
            if a[i-1] == b[j-1] {
                curr[j] = prev[j-1];
            } else {
                curr[j] = std::cmp::min(curr[j-1], std::cmp::min(prev[j], prev[j-1])) + 1
            }
        }
        prev = curr;
    }
    println!("{}", prev[b.len()]);
}