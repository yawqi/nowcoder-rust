fn main() {
    let mut ret = vec![];
    loop {
        let mut s = String::new();
        std::io::stdin()
            .read_line(&mut s);
        let mut n: u32 = s.trim().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut r = 0;
        while n >= 3 {
            let remain = n / 3;
            n = n % 3 + remain;
            r += remain;
        }
        if n == 2 {
            r += 1;
        }
        ret.push(r);
    }

    for r in ret {
        println!("{}", r);
    }
}