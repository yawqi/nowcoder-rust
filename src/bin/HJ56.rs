fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n);
    let n = n.trim().parse::<u32>().unwrap();
    let mut count = 0;
    for i in 24..n {
        let mut sum = 0;
        for j in 2..=i/2 {
            if i % j == 0 {
                let k = i / j;
                if k < j {
                    break;
                }
                sum += j;
                if k != j {
                    sum += k;
                } else {
                    break;
                }
            }
        }
        if sum == i {
            count += 1;
        }
    }
    println!("{}", count);
}