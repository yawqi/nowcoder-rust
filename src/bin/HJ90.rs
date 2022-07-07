fn main() {
    let mut results = vec![];
    loop {
        let mut ip = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut ip).unwrap();
        if bytes == 0 {
            break;
        }
        
        let ips = ip.trim().split(".").collect::<Vec<_>>();
        let mut r = ips.len() == 4;
        if r {
            r = !ips.iter()
                    .any(|n| {
                        let contain_others = n.contains(|c: char| !c.is_digit(10));
                        n.len() > 1 && n.get(0..=0).unwrap() == "0" || contain_others
                    });
        }
        let ip = ips.iter().map(|n| n.parse::<u32>()).collect::<Vec<_>>();
        if r {
            r = ip.iter()
                .all(|n| {
                    if n.is_ok() {
                        let num = n.clone().unwrap();
                        num <= 255
                    } else {
                        false
                    }
                });
        }
        results.push(r);
    }

    for r in results {
        if r {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}