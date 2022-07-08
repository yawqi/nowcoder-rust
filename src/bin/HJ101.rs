fn main() {
    let mut n = String::new();
    let mut nums = String::new();
    let mut order = String::new();
    std::io::stdin()
        .read_line(&mut n);
    std::io::stdin()
        .read_line(&mut nums);
    std::io::stdin()
        .read_line(&mut order);
    let mut nums = nums.trim().split(" ").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<_>>();
    let order = order.trim();

    nums.sort_by(|a, b| if order == "0" {
        a.cmp(b)
    } else {
        b.cmp(a)
    });

    for num in nums {
        print!("{} ", num);
    }
}