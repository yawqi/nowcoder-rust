fn main() {
    let mut n = String::new();
    let mut nums = String::new();

    std::io::stdin()
        .read_line(&mut n);
    std::io::stdin()
        .read_line(&mut nums);
    let mut nums = nums.trim().split(" ").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let mut max_count = 1;
    let mut counts = vec![1; nums.len()];
    for i in 1..nums.len() {
        let curr = nums[i];
        for p in 0..i {
            let prev = nums[p];
            if prev < curr {
                counts[i] = std::cmp::max(counts[i], counts[p] + 1);
            }
        }
        max_count = std::cmp::max(max_count, counts[i]);
    }
    println!("{}", max_count);
}