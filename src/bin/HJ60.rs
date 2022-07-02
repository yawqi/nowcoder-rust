fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n);
    let n = n.trim().parse::<u32>().unwrap();
    if n == 4 {
        println!("2");
        println!("2");
    }
    for i in (3..(n+2)/2).step_by(2).rev() {
        if is_prime(i) && is_prime(n-i) {
            println!("{}", i);
            println!("{}", n-i);
            return;
        }
    }
}

fn is_prime(n: u32) -> bool {
    if n % 2 == 0 || n < 2 {
        return false;
    }

    for i in (3..).step_by(2).take_while(|x| x * x <= n) {
        if n % i == 0 {
            return false;
        }
    }
    true
}