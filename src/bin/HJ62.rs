fn main() {
    let mut result = vec![];
    loop {
        let mut n = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut n).unwrap();
        if bytes == 0 {
            break;
        }
        let mut n = n.trim().parse::<u32>().unwrap();
        let mut count = 0;
        while n != 0 {
            n &= n - 1;
            count += 1;
        }
        result.push(count);
    }
    for c in result {
        println!("{}", c);
    }
}