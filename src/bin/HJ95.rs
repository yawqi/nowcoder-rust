const v: &[char] = &['零', '壹', '贰', '叁', '肆', '伍', '陆', '柒', '捌', '玖', '拾'];
fn main() {
    let mut money = String::new();
    std::io::stdin()
        .read_line(&mut money);
    let money = money.trim().parse::<f64>().unwrap();
    let mut s = "人民币".to_string();
    if money >= 10000.0 {
        s.push_str(&parse_10ks(money));
    }
    let hunderds = parse_hunders(money);
    let others = parse_others(money);
    if hunderds == "零" && !others.is_empty() {
        s.push_str(&others);
    } else {
        s.push_str(&hunderds);
        s.push('元');
        if others.is_empty() {
            s.push('整');
        } else {
            s.push_str(&others);
        }
    }
    println!("{}", s);
}

fn parse_10ks(num: f64) -> String {
    let mut s = "".to_string();
    let mut num = num.floor() as u64;
    let num = (num / 10000) as u64;
    let a = parse_hunders(num as f64);
    s.push_str(&a);
    s.push('万');
    s
}

fn parse_hunders(num: f64) -> String {
    let mut s = "".to_string();
    let orig = num;
    let mut num = num.floor() as u64;
    num %= 10000;
    if num == 0 {
        return "零".to_string();
    }

    if num / 1000 != 0 {
        s.push_str(&format!("{}仟", v[(num/1000) as usize]));
    } 
    
    num %= 1000;

    if num / 100 != 0 {
        s.push_str(&format!("{}佰", v[(num/100) as usize]));
    }  else if !s.contains('零') && orig >= 1000.0 {
        s.push('零');
    }
    num %= 100;
    
    if num / 10 != 0 {
        if num / 10 == 1 {
            s.push_str(&format!("拾"));
        } else {
            s.push_str(&format!("{}拾", v[(num/10) as usize]));
        }
    } else if !s.contains('零') && orig >= 100.0 {
        s.push('零');
    }
    num %= 10;

    if num != 0 {
        s.push(v[num as usize]);
    }
    s
}

fn parse_others(num: f64) -> String {
    let mut s = "".to_string();
    if num - num.floor() != 0.0 {
        let count = ((num - num.floor()) * 101.0) as u64;
        println!("{}", num - num.floor());
        println!("{}", count);
        if count / 10 != 0 {
            s.push_str(&format!("{}角", v[(count/10) as usize]));
        }

        if count % 10 != 0 {
            s.push_str(&format!("{}分", v[(count%10) as usize]));
        }
    } else {
        s.push('整');
    }
    s
}