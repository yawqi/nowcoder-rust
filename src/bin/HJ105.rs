fn main() {
    let mut neg_counts = 0;
    let mut sum = 0.0;
    let mut other_counts = 0.0;
    loop {
        let mut num = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut num).unwrap();
        if bytes == 0 {
            break;
        }
        let num = num.trim().parse::<i32>().unwrap();
        if num < 0 {
            neg_counts += 1;
        } else {
            other_counts += 1.0;
            sum += num as f64;
        }
    }
    println!("{}", neg_counts);
    if other_counts == 0.0 {
        println!("0.0");
    } else {
        println!("{:.1}", sum / other_counts);
    }
}