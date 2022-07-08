use std::io::Read;

fn main() {
    let mut remains = vec![0;4];
    let mut goods = vec![0;6];
    let prices = vec![2,3,4,5,8,6];
    let mut curr = 0;
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);
    let commands = s.trim()
                    .split(";")
                    .collect::<Vec<_>>();
    for command in commands {
        if command.is_empty() {
            continue;
        }
        let cmd = command.split(" ").collect::<Vec<_>>();
        let c = cmd[0];
        match c.chars().nth(0).unwrap() {
            'r' => {
                init(&mut remains, &mut goods, &cmd[1..]);
            },
            'p' => {
                let payment = cmd[1].parse::<u32>().unwrap();
                if payment != 1 && payment != 2 && payment != 5 && payment != 10 {
                    println!("E002:Denomination error");
                    continue;
                }
                let change = remains[0] + remains[1] * 2;
                if change < payment && payment > 2 {
                    println!("E003:Change is not enough, pay fail");
                    continue;
                }

                let all_sold = goods.iter().all(|count| *count == 0);
                if all_sold {
                    println!("E005:All the goods sold out");
                    continue;
                }
                match payment {
                    1 => remains[0] += 1,
                    2 => remains[1] += 1,
                    5 => remains[2] += 1,
                    10 => remains[3] += 1,
                    _ => {},
                }

                curr += payment;
                println!("S002:Pay success,balance={}", curr);
            },
            'b' => {
                let good = (cmd[1].chars().nth(1).unwrap().to_digit(10).unwrap() - 1) as usize;
                if good > 5 {
                    println!("E006:Goods does not exist");
                    continue;
                }

                if goods[good] == 0 {
                    println!("E007:The goods sold out");
                    continue;
                }

                if curr < prices[good] {
                    println!("E008:Lack of balance");
                    continue;
                }

                curr -= prices[good];
                goods[good] -= 1;
                println!("S003:Buy success,balance={}", curr);
            },
            'c' => {
                if curr == 0 {
                    println!("E009:Work failure");
                    continue;
                }

                let mut ret = vec![0;4];
                if curr >= 10 && remains[3] > 0 {
                    let count = std::cmp::min(curr/10, remains[3]);
                    curr -= count * 10;
                    remains[3] -= count;
                    ret[3] += count;
                }

                if curr >= 5 && remains[2] > 0 {
                    let count = std::cmp::min(curr/5, remains[2]);
                    curr -= count * 5;
                    remains[2] -= count;
                    ret[2] += count;
                }

                if curr >= 2 && remains[1] > 0 {
                    let count = std::cmp::min(curr/2, remains[1]);
                    curr -= count * 2;
                    remains[1] -= count;
                    ret[1] += count;
                }

                if curr >= 1 && remains[0] > 0 {
                    let count = std::cmp::min(curr, remains[0]);
                    curr -= count;
                    remains[0] -= count;
                    ret[0] += count;
                }
                curr = 0;
                println!("1 yuan coin number={}", ret[0]);
                println!("2 yuan coin number={}", ret[1]);
                println!("5 yuan coin number={}", ret[2]);
                println!("10 yuan coin number={}", ret[3]);
            },
            'q' => {
                if cmd.len() == 1 {
                    println!("E010:Parameter error");
                    continue;
                }
                let arg = cmd[1].parse::<u32>();
                if arg.is_err() {
                    println!("E010:Parameter error");
                    continue;
                }
                let arg = arg.unwrap();
                match arg {
                    0 => {
                        println!("A1 {} {}",prices[0],goods[0]);
                        println!("A2 {} {}",prices[1],goods[1]);
                        println!("A3 {} {}",prices[2],goods[2]);
                        println!("A4 {} {}",prices[3],goods[3]);
                        println!("A5 {} {}",prices[4],goods[4]);
                        println!("A6 {} {}",prices[5],goods[5]);
                    },
                    1 => {
                        println!("1 yuan coin number={}", remains[0]);
                        println!("2 yuan coin number={}", remains[1]);
                        println!("5 yuan coin number={}", remains[2]);
                        println!("10 yuan coin number={}", remains[3]);
                    },
                    _ => {
                        println!("E010:Parameter error");
                    },
                }
            },
            _ => {},
        }
    }
}

fn init(remains: &mut Vec<u32>, goods: &mut Vec<u32>, args: &[&str]) {
    let goods_str = args[0];
    let remains_str = args[1];
    *remains = remains_str.split("-").map(|n| n.parse::<u32>().unwrap()).collect();
    *goods = goods_str.split("-").map(|n| n.parse::<u32>().unwrap()).collect();
    println!("S001:Initialization is successful");
}