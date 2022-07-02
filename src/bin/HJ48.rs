use std::collections::HashMap;
fn main() {
    let mut nums = String::new();
    std::io::stdin()
        .read_line(&mut nums).unwrap();
    
    let nums: Vec<_> = nums.trim().split(" ").map(|n| n.parse::<i32>().unwrap()).collect();
    let n = nums[0];
    let mut list = vec![nums[1]];
    let mut map = HashMap::new();
    map.insert(nums[1], 0);
    for i in 1..n as usize {
        let mut ins = nums[2 + 2 * i];
        let mut after = nums[2 + 2 * i + 1];
        let idx = map.get(&after).unwrap();
        list.insert(*idx + 1 as usize, ins);
    } 

    let idx = map.get(&nums[nums.len()-1]).unwrap();
    list.remove(*idx);
    let list = list.into_iter().map(|n| n.to_string()).collect::<Vec<_>>();

}