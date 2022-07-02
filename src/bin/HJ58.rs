fn main() {
    let mut exists = [0; 10001];
    let mut nk = String::new();
    let mut nums = String::new();
    std::io::stdin()
        .read_line(&mut nk);
    std::io::stdin()
        .read_line(&mut nums);
    let nk = nk.trim().split(" ").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
    nums.trim().split(" ").map(|n| n.parse::<usize>().unwrap()).for_each(|n| exists[n] += 1);
    let mut count = 0;
    let mut index = 1;
    while count < nk[1] {
        if exists[index] > 0 {
            count += exists[index];
            for _ in 0..exists[index] {
                print!("{} ", index);
            }
        }
        index += 1;
    }
}