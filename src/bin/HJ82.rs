use std::io::Read;

fn main() {
    let mut results = vec![];
    loop {
        let mut number = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut number).unwrap();
        if bytes == 0 {
            break;
        }
        let numbers = number.trim().split("/").map(|n| n.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let mut son = numbers[0];
        let mut mother = numbers[1];
        let mut mothers = vec![];
        while son != 0 {
            let gcd = gcd(mother, son);
            mother /= gcd;
            son /= gcd;
            
            let p = mother / son;
            let r = mother % son;
            mothers.push(p+1);
            son -= r;
            mother *= (p + 1);
        }
        let mothers = mothers.into_iter().map(|n| format!("1/{}", n)).collect::<Vec<_>>();
        results.push(mothers.join("+"));
    }
    for r in results {
        println!("{}", r);
    }
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}