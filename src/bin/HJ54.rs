fn main() {
    let mut ops = vec![];
    let mut operands = vec![0];
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s).unwrap();
    let chs = s.trim().chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut is_neg = false;
    while i < chs.len() {
        let ch = chs[i];
        match ch {
            ch if ch.is_digit(10) => {
                let mut idx = i + 1;
                while idx < chs.len() && chs[idx].is_digit(10) {
                    idx += 1;
                }
                let mut operand = chs[i..idx].iter().fold(0, |mut a, ch| {
                    a *= 10;
                    a += ch.to_digit(10).unwrap() as i32;
                    a
                });
                if is_neg {
                    operand = -operand;
                    is_neg = false;
                }
                operands.push(operand);
                i = idx - 1;
            },
            '(' | '*' | '/' => {
                ops.push(ch);
            },
            '+' | '-' => {
                if ch == '-' {
                    is_neg = !is_neg;
                }
                while !ops.is_empty() {
                    let last = ops.last().unwrap();
                    if *last == '*' || *last == '/' {
                        let n2 = operands.pop().unwrap();
                        let n1 = operands.pop().unwrap();
                        let num = match *last {
                            '*' => n1 * n2,
                            '/' => n1 / n2,
                            _ => {
                                return;
                            }
                        };
                        operands.push(num);
                    } else {
                        break;
                    }
                    ops.pop();
                }
                ops.push('+');
            }
            ')' => {
                while !ops.is_empty() {
                    let last = ops.pop().unwrap();
                    if last == '(' {
                        break;
                    }
                    let n2 = operands.pop().unwrap();
                    let n1 = operands.pop().unwrap();
                    let mut num = match last {
                        '+' => n1 + n2,
                        '-' => n1 - n2,
                        '*' => n1 * n2,
                        '/' => n1 / n2,
                        _ => return,
                    };
                    if is_neg {
                        num = -num;
                        is_neg = false;
                    }
                    operands.push(num);
                }
            },
            _ => {

            },
        }
        i += 1;
    }
    let mut n1 = operands.pop().unwrap();
    while !ops.is_empty() {
        let op = ops.pop().unwrap();
        let n2 = operands.pop().unwrap();
        n1 = match op {
            '+' => n1 + n2,
            '-' => n1 - n2,
            '*' => n1 * n2,
            '/' => n1 / n2,
            _ => return,
        };
    }
    println!("{}", n1);
}