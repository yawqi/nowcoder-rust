fn main() {
    let mut nums = String::new();
    std::io::stdin()
        .read_line(&mut nums);
    let nums = nums.trim().split(" ").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>();
    let mut is_ok = false;
    for idx in 0..4 { 
        let mut new_num = nums.clone();
        new_num.remove(idx);
        let a = nums[idx];
        calculate(a, &new_num, &mut is_ok);
    }
    println!("{}", is_ok);
}

fn calculate(a: i32, b: &[i32], is_ok: &mut bool) {
    if *is_ok || b.is_empty() {
        if a == 24 {
            *is_ok = true;
        }
        return;
    }
    
    for idx in 0..b.len() { 
        let num = b[idx];
        let mut nb = b.iter().collect::<Vec<_>>();
        nb.remove(idx);
        calculate(a + num, &b[1..], is_ok);
        calculate(a - num, &b[1..], is_ok);
        calculate(a * num, &b[1..], is_ok);
        calculate(a / num, &b[1..], is_ok);
    }
}