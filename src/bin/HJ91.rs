fn main() {
    let mut n1_n2 = String::new();
    std::io::stdin()
        .read_line(&mut n1_n2);
    let n1_n2 = n1_n2.trim().split(" ").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let n1 = n1_n2[0];
    let n2 = n1_n2[1];
    let n = n1 + n2;
    let mut sum = 1;
    for i in (n2+1..=n).rev() {
        sum *= i;
    }

    for i in (2..=n1) {
        sum /= i;
    }

    println!("{}", sum);
}