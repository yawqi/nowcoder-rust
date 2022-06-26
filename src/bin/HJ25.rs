use std::collections::BTreeSet;
use std::collections::VecDeque;
fn main() {
    loop {
        let mut I = String::new();
        let mut R = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut I).unwrap();
        if bytes == 0 {
            break;
        }
        std::io::stdin().read_line(&mut R).unwrap();
        let mut i_vec: Vec<_>= I.trim().split(' ').collect();
        let mut r_vec: Vec<_>= R.trim().split(' ').collect();

        let mut i_records: Vec<String> = i_vec[1..].into_iter()
                            .map(|i| i.to_string())
                            .collect();
        let mut r_records: Vec<String> = r_vec[1..].into_iter()
                                .map(|r| r.to_string())
                                .collect();
        let mut r_set = BTreeSet::new();
        for record in r_records {
            r_set.insert(record.parse::<u32>().unwrap());
        }
        
        let mut result = VecDeque::new();
        for query in r_set {
            let mut res = VecDeque::new();
            let mut count = 0;
            for (idx, i) in i_records.iter().enumerate() {
                if i.contains(&query.to_string()) {
                    res.push_back(idx.to_string());
                    res.push_back(i.to_string());
                    count += 1;
                }
            }
            if count == 0 {
                continue;
            }
            res.push_front(count.to_string());
            res.push_front(query.to_string());
            result.extend(res);
        }
        result.push_front(result.len().to_string());
        let result: Vec<_> = result.into_iter().collect();
        let s = result.join(" ");
        println!("{}", s);
    }
}