fn main() {
    let mut nums = String::new();
    std::io::stdin()
        .read_line(&mut nums);
    let nums = nums.trim().split(" ").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let n1 = nums[0];
    let n2 = nums[1];
    println!("{}", lcm(n1, n2));
}

fn lcm(n1: u32, n2: u32) -> u32 {
    n1 * n2 / gcd(n1, n2)
}

fn gcd(n1: u32, n2: u32) -> u32 {
    let mut min = std::cmp::min(n1, n2);
    let mut max = std::cmp::max(n1, n2);

    loop {
        let remain = max % min;
        max = min;
        if remain == 0 {
            break;
        }
        min = remain;
    }
    max
}