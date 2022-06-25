use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::HashMap;
fn main() {
    let mut deque: VecDeque<String> = VecDeque::new();
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut set: HashSet<String> = HashSet::new();
    loop {
        let mut s = String::new();
        let bytes = std::io::stdin()
            .read_line(&mut s).unwrap();
        if bytes == 0 {
            break;
        }
        let v: Vec<_> = s.trim()
                .split(' ')
                .collect();
        let (filename, line) = (v[0], v[1]);
        let mut filename = filename.split('\\').last().unwrap().to_string();
        let len = filename.len();
        if len > 16 {
            filename = filename[len-16..].to_string();
        }
        filename.push(' ');
        filename.push_str(line);
        if map.contains_key(&filename) {
            *map.get_mut(&filename).unwrap() += 1;
        } else {
            if set.contains(&filename) {
                continue;
            }
            if map.len() >= 8 {
                let victim = deque.pop_front().unwrap();
                map.remove(&victim);
                set.insert(victim);
            }
            deque.push_back(filename.clone());
            map.insert(filename.clone(), 1);
        }
    }

    for record in deque {
        println!("{} {}", record, map.get(&record).unwrap());
    }

}