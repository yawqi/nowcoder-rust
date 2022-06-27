fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .unwrap();
    let n: usize = n.trim().parse().unwrap();
    let mut odds = vec![];
    let mut evens = vec![];
    let mut nums = String::new();
    std::io::stdin()
        .read_line(&mut nums)
        .unwrap();
    nums.trim()
        .split(" ")
        .map(|n| n.parse::<usize>().unwrap())
        .for_each(|n| {
            if n % 2 == 0 {
                evens.push(n);
            } else {
                odds.push(n);
            }
        });
    let mut matches = vec![vec![]; evens.len()];
    for (i, even) in evens.iter().enumerate() {
        for (j, odd) in odds.iter().enumerate() {
            if is_prime(even + odd) {
                matches[i].push(j);
            }
        }
    }

    let mut choosen = vec![-1; odds.len()];
    let mut count = 0;
    for idx in 0..evens.len() {
        let mut access = vec![false; evens.len()];
        if choose(idx, &mut matches, &mut choosen, &mut access) {
            count += 1;
        }
    }

    println!("{}", count);
}

fn choose(idx: usize, matches: &mut Vec<Vec<usize>>, choosen: &mut Vec<i32>, access: &mut Vec<bool>) -> bool {
   for index in matches[idx].clone() {
        if choosen[index] == -1 {
            choosen[index] = idx as i32;
            return true;
        }

        if !access[choosen[index] as usize] && choosen[index] as usize != idx {
            access[choosen[index] as usize] = true;
            if choose(choosen[index] as usize, matches, choosen, access) {
                choosen[index] = idx as i32;
                return true;
            }
        }
    }
    false
}

fn is_prime(n: usize) -> bool {
    if n <= 2 || n % 2 == 0 {
        false
    } else {
        for i in (3..).step_by(2).take_while(|x| x * x <= n) {
            if n % i == 0 {
                return false;
            }
        }
        true
    }
}