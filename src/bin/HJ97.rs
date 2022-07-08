fn main() {
    let mut n = String::new();
    let mut nums = String::new();

    std::io::stdin()
        .read_line(&mut n);
    std::io::stdin()
        .read_line(&mut nums);
    
    let mut neg_count = 0;
    let mut pos_count = 0;
    let mut pos_sum = 0;
    let nums = nums
            .trim()
            .split(" ")
            .map(|n| n.parse::<i32>().unwrap())
            .for_each(|n| {
                if n < 0 {
                    neg_count += 1;
                } else if n > 0 {
                    pos_count += 1;
                    pos_sum += n;
                }
            });
    
    let mut average = (pos_sum as f64 / pos_count as f64);
    if average.is_nan() {
        average = 0.0;
    }
    println!("{} {:.1}", neg_count, average);
}