fn main() {
    let mut n1 = String::new();
    let mut n2 = String::new();

    std::io::stdin()
        .read_line(&mut n1);
    std::io::stdin()
        .read_line(&mut n2);
    let mut v1 = n1.trim().chars().rev().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
    let mut v2 = n2.trim().chars().rev().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>();
    if v1.len() < v2.len() {
        v1.extend(vec![0; v2.len() - v1.len()]);
    } else {
        v2.extend(vec![0; v1.len() - v2.len()]);
    }
    let mut addone = false;
    for i in 0..v1.len() {
        let a = v1[i];
        let b = v2[i];
        let mut sum = a + b;
        if addone {
            sum += 1;
        }
        if sum >= 10 {
            addone = true;
            sum %= 10;
        } else {
            addone = false;
        }
        v2[i] = sum;
    }
    if addone {
        v2.push(1);
    }

    let ret = v2.into_iter().rev().map(|n| (n + '0' as u8) as char).collect::<String>();
    println!("{}", ret);
}