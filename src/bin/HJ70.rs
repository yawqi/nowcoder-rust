
fn main() {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();

    let mut matrixes = vec![];
    for _ in 0..n {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line).unwrap();

        let matrix = line
                .trim()
                .split(" ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
        matrixes.push(matrix);
    }
    let mut exp = String::new();
    std::io::stdin()
        .read_line(&mut exp).unwrap();
    let mut sum = 0;
    let mut ops = vec![];
    let mut operands: Vec<Vec<usize>> = vec![];
    for e in exp.trim().chars() {
        match e {
            '(' => {
                ops.push(e);
            },
            ')' => {
                let mut tmp = vec![];
                while let Some(e) = ops.pop() {
                    if e == '(' {
                        break;
                    }
                    tmp.push(operands.pop().unwrap());
                }
                while let Some(mut op1) = tmp.pop() {
                    if let Some(op2) = tmp.pop() {
                        sum += op1[0]*op2[1]*op1[1];
                        op1[1] = op2[1];
                        tmp.push(op1);
                    } else {
                        operands.push(op1);
                        ops.push('A');
                    }
                }
            },
            e if e.is_ascii_uppercase() => {
                ops.push(e);
                operands.push(matrixes[(e as u8 - 'A' as u8) as usize].clone());
            },
            _ => {}
        }
    }
    println!("{}", sum);
}
