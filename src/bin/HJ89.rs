fn main() {
    let mut cards = String::new();
    std::io::stdin()
        .read_line(&mut cards);
    let cards = cards.trim().split(" ").collect::<Vec<_>>();
    let mut nums = vec![];
    for card in cards {
        let num = match card.parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                match card {
                    "A" => 1,
                    "J" => 11,
                    "Q" => 12,
                    "K" => 13,
                    _ => {
                        println!("ERROR");
                        return;
                    }
                }
            },
        };
        nums.push(num);
    }

    for i in 0..4 {
        let cur = nums[i];
        for j in 0..4 {
            if j == i {
                continue;
            }
            for k in 0..4 {
                if k == j || k == i {
                    continue;
                }
                for o in 0..4 {
                    if o == k || o == j || o == i {
                        continue;
                    }
                    let mut nums2 = vec![nums[j], nums[k], nums[o]];
                    let mut operands = vec![cur];
                    let mut ops = vec![];
                    if calc(cur, &mut nums2[..], &mut operands, &mut ops) {
                        let operands = operands.into_iter().map(|n| {
                            match n {
                                2..=10 => n.to_string(),
                                1 => "A".to_string(),
                                11 => "J".to_string(),
                                12 => "Q".to_string(),
                                13 => "K".to_string(),
                                _ => "".to_string(),
                            }
                        }).collect::<Vec<_>>();
                        let mut s = operands[0].clone();
                        let s1 = operands.into_iter()
                                .skip(1)
                                .zip(ops.into_iter())
                                .map(|(operand, op)| {
                                    let mut s = op.to_string();
                                    s.push_str(&operand);
                                    s
                                })
                                .collect::<Vec<String>>();
                        s.push_str(&s1.join(""));
                        println!("{}", s);
                        return;
                    }
                }
            }
        }
    }
    println!("NONE");
}

fn calc(cur: i32, nums: &mut [i32], operands: &mut Vec<i32>, ops: &mut Vec<char>) -> bool {
    if nums.len() == 0 {
        if cur == 24 {
            return true;
        }
        return false;
    }

    operands.push(nums[0]);
    ops.push('+');
    if calc(cur + nums[0], &mut nums[1..], operands, ops) {
        return true;
    }
    ops.pop();

    ops.push('-');
    if calc(cur - nums[0], &mut nums[1..], operands, ops) {
        return true;
    }
    ops.pop();

    ops.push('*');
    if calc(cur * nums[0], &mut nums[1..], operands, ops) {
        return true;
    }
    ops.pop();

    ops.push('/');
    if calc(cur / nums[0], &mut nums[1..], operands, ops) {
        return true;
    }
    ops.pop();

    operands.pop();
    false
}