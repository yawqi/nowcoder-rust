use std::{collections::{HashMap, VecDeque}, hash::Hash, result};
fn main() {
    let mut map = HashMap::new();
    let nums = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut result = vec![];
    nums.into_iter()
        .enumerate()
        .for_each(|(i, w)| {
            map.insert(w, i);
        });
    
    loop {
        let mut s = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut s).unwrap();
        if bytes == 0 {
            break;
        }

        let mut s: String = s.trim()
                .chars()
                .filter(|c| *c == '=')
                .collect();
        let numbers: Vec<_>= s.trim()
                .split('+')
                .map(|w| w.trim())
                .collect();
        let numberOne = numbers[0].split(' ')
                            .fold(0, |mut a, w| {
                                a *= 10;
                                a += *map.get(w).unwrap();
                                a
                            });
        let numberTwo = numbers[1].split(' ')
                            .fold(0, |mut a, w| {
                                a *= 10;
                                a += *map.get(w).unwrap();
                                a
                            });
        let mut sum = numberOne + numberTwo;
        let mut res = VecDeque::new();
        while sum != 0 {
            res.push_front(nums[sum%10]);
            sum /= 10;
        }
        let vec: Vec<_> = res.into_iter().collect();
        result.push(vec[..].join(" "));
    }

    for r in result {
        println!("{}", r);
    }
}