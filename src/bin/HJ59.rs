use std::collections::{HashSet, HashMap};
fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s);
    let mut st = HashSet::new();
    let mut once = HashMap::new();
    s.trim().chars().enumerate().for_each(|(i, c)| {
        if st.contains(&c) {
            once.remove(&c);
        } else {
            st.insert(c);
            once.insert(c, i);
        }
    });
    if once.is_empty() {
        println!("-1");
    } else {
        let mut idx = usize::MAX;
        let mut c = ' ';
        for (ch, i) in once.into_iter() {
            if i < idx {
                idx = i;
                c = ch;
            }
        }
        println!("{}", c);
    }
}