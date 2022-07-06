use std::f32::consts::E;

fn main() {
    let mut count = vec![];
    let mut nums = vec![];
    for _ in 0..2 {
        let mut n = String::new();
        let mut num = String::new();
        std::io::stdin()
            .read_line(&mut n);
        std::io::stdin()
            .read_line(&mut num);
        count.push(n.trim().parse::<usize>().unwrap());
        nums.push(num.trim().split(" ").map(|n| n.parse::<i32>().unwrap()).collect::<Vec<_>>());
    }
    let mut num2 = nums.pop().unwrap();
    let mut num1 = nums.pop().unwrap();
    num1.append(&mut num2);
    num1.sort();
    num1.dedup();
    for n in num1 {
        print!("{}", n);
    }
}