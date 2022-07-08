fn main() {
    let mut f = String::new();
    std::io::stdin()
        .read_line(&mut f);
    let mut f = f.trim().parse::<f64>().unwrap();
    let mut is_neg = false;
    if f < 0.0 {
        f = -f;
        is_neg = true;
    }

    let mut start = 0.0;
    if f > 1.0 {
        start = 1.1;
    } else {
        start = 0.0;
    }
    println!("{}", f);
    let mut prev_diff = 20.0;
    let mut prev = start;
    loop {
        let r = start * start * start;
        println!("{} {}", start, r);
        if (r - f).abs() > prev_diff || start >= 3.0 {
            break;
        }
        prev_diff = (r-f).abs();
        prev = start;
        start += 0.1;
    }
    if is_neg {
        prev = -prev;
    }
    println!("{:.1}", prev);
}