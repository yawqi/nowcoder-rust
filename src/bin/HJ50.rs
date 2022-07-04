use std::io::Read;

fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s).unwrap();
    let sum = evaluate(s.trim());
    println!("{}", sum.0);
}

fn evaluate(s: &str) -> (i32, usize) {
    let mut operands: Vec<i32> = vec![];
    let mut ops = vec![];
    let v = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    let mut is_operand = false;
    let mut should_neg = false;
    let mut prev_op = ' ';
    let mut ret_len = 0;
    while i < v.len() {
        let ch = v[i];
        match ch {
            ch if ch.is_digit(10) => {
                let mut i2 = i + 1;
                while i2 < v.len() && v[i2].is_digit(10){
                    i2 += 1;
                }
                let mut n2 = v[i..i2].iter().fold(0, |mut n, ch| {
                    n *= 10;
                    n += ch.to_digit(10).unwrap() as i32;
                    n
                });
                i = i2 - 1;
                if should_neg {
                    n2 = -n2;
                    should_neg = false;
                }
                if prev_op == '*' || prev_op == '/' {
                    let n1 = operands.pop().unwrap();
                    match prev_op {
                        '*' => operands.push(n1*n2),
                        '/' => operands.push(n1/n2),
                        _ => {},
                    }
                    prev_op = ' ';
                } else {
                    operands.push(n2);
                }
                is_operand = true;
            }
            '{' | '(' | '[' => {
                let (n2, len) = evaluate(&s[i+1..]);
                i += len + 1;
                is_operand = true;
                if prev_op == '*' || prev_op == '/' {
                    let n1 = operands.pop().unwrap();
                    match prev_op {
                        '*' => operands.push(n1*n2),
                        '/' => operands.push(n1/n2),
                        _ => {},
                    }
                    prev_op = ' ';
                } else {
                    operands.push(n2);
                }
            },
            '}' | ')' | ']' => {
                ret_len = i;
                break;
            },
            '+' | '-' => {
                if !is_operand && ch == '-' {
                    should_neg = true;
                } else {
                    ops.push(ch);
                }
                is_operand = false;
            }
            '*' | '/' => {
                ops.push(ch);
                is_operand = false;
                prev_op = ch;
            }
            _ => {}
        }
        i += 1;
    }
    operands = operands.into_iter().rev().collect();
    ops = ops.into_iter().rev().collect();
    let mut n1 = operands.pop().unwrap();
    while let Some(op) = ops.pop() {
        match op {
            '-' => {
                let n2 = operands.pop().unwrap();
                n1 -= n2;
            },
            '+' => {
                let n2 = operands.pop().unwrap();
                n1 += n2
            },
            _ => {},
        }
    }
    (n1, ret_len)
}
// 5-3+9*6*(6-10-2)