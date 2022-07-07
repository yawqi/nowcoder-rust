fn main() {
    let mut results = vec![];
    let mut lens = vec![];
    loop {
        let mut s = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut s).unwrap();
        if bytes == 0 {
            break;
        }

        let v = s.trim().chars().collect::<Vec<_>>();

        let mut nums = vec![];
        let mut max_len = 0;
        let mut len = 0;
        for i in 0..v.len() {
            let c = v[i];
            if c.is_digit(10) {
                len += 1;
            }
            if !c.is_digit(10) || i == v.len() - 1 {
                if len == max_len {
                    if i == v.len() - 1 {
                        nums.push(s[i-len+1..=i].to_string());
                    } else {
                        nums.push(s[i-len..i].to_string());
                    }
                } else if len > max_len{
                    max_len = len;
                    nums.clear();
                    if i == v.len() - 1 {
                        nums.push(s[i-len+1..=i].to_string());
                    } else {
                        nums.push(s[i-len..i].to_string());
                    }
                }
                len = 0;
            }
        }
        lens.push(max_len);
        results.push(nums.join(""));
    }

    for i in 0..lens.len() {
        println!("{},{}", results[i], lens[i]);
    }
}