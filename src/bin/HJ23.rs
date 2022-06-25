fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);
    let s: String = s.trim().chars().collect();
    let mut nums = [0; 26];
    s.chars()
        .for_each(|c| {
            nums[(c as u8- 'a' as u8) as usize] += 1;
        });
    let mut smallest = usize::MAX;
    nums.iter()
        .for_each(|num| {
            if *num != 0 {
                smallest = std::cmp::min(*num, smallest);
            }
        });
    let s: String = s.chars()
        .filter(|c| nums[(*c as u8 - 'a' as u8) as usize] != smallest)
        .collect();
    println!("{}", s);
}