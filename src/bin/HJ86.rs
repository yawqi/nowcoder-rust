fn main() {
    let mut results = vec![];
    loop {
        let mut n = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut n).unwrap();
        if bytes == 0 {
            break;
        }

        let mut n = n.trim().parse::<u32>().unwrap();
        let mut count = 0;
        let mut max = 0;
        while n != 0 {
            if n & 1 == 0 {
                count = 0;
            } else {
                count += 1;
                max = std::cmp::max(max, count);
            }

            n >>= 1;
        }
        results.push(max);
    }
    for r in results {
        println!("{}", r);
    }
}