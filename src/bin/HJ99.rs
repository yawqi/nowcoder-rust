fn main() {
    let mut count = 0;
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n);
    let n = n.trim().parse::<u64>().unwrap();
    let mut mymod = 10;
    for i in 0..=n {
        if i >= mymod {
            mymod *= 10;
        } 
        if ((i * i) % mymod) == i {
            count += 1;
        }
    }

    println!("{}", count);
}