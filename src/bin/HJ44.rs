use std::{io::Read, collections::HashSet};

fn main() {
    let mut soduku = vec![];
    let mut zeros = vec![];
    for _ in 0..9 {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line).unwrap();
        let r = line.trim()
            .split(" ")
            .map(|n| n.parse::<i8>().unwrap())
            .collect::<Vec<i8>>();
        soduku.push(r)
    };
    let mut col_available = vec![vec![true; 9]; 9];
    let mut row_available = vec![vec![true; 9]; 9];
    let mut square_available = vec![vec![true; 9]; 9];
    for r in 0..9 {
        for c in 0..9 {
            let num = soduku[r][c];
            if num != 0 {
                row_available[r][num as usize - 1] = false; 
                col_available[c][num as usize - 1] = false; 
                square_available[(r/3)*3+c/3][num as usize -1] = false; 
            } else {
                zeros.push((r as usize ,c as usize));
            }
        }
    }
    println!("{:?}", zeros);
    // println!("{:?}", row_available);
    // println!("{:?}", col_available);
    // println!("{:?}", square_available);
    matches(&mut zeros, &mut soduku, &mut row_available, &mut col_available, &mut square_available);
    for l in soduku {
        let mut line = String::new();
        for n in l {
            line.push((n as u8 + '0' as u8) as char);
            line.push(' ');
        }
        println!("{}", line.trim());
    }
}

fn matches(zeros: &mut Vec<(usize, usize)>, soduku: &mut Vec<Vec<i8>>, row_available: &mut Vec<Vec<bool>>, col_available: &mut Vec<Vec<bool>>, square_available: &mut Vec<Vec<bool>>) -> bool {
    if zeros.len() == 0 {
        return true;
    }
    let (r, c) = zeros.pop().unwrap();
    for num in 1..=9 {
        if row_available[r][num-1] && col_available[c][num-1] && square_available[(r/3)*3+c/3][num-1] {
            row_available[r][num-1] = false;
            col_available[c][num-1] = false;
            square_available[(r/3)*3+c/3][num-1] = false;
            soduku[r][c] = num as i8;
            let ret = matches(zeros, soduku, row_available, col_available, square_available);
            if ret {
                println!("({},{}) found {}", r, c, num);
                return ret;
            }
            row_available[r][num-1] = true;
            col_available[c][num-1] = true;
            square_available[(r/3)*3+c/3][num-1] = true;
            soduku[r][c] = 0 as i8;
        }
    }
    zeros.push((r, c));
    return false;
}