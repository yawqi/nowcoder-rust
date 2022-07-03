use std::collections::{HashSet, HashMap};
fn main() {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s).unwrap();
    let v = s.trim()
                .split(" ")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
    let apples = v[0];
    let plates = v[1];
    let mut curr = vec![];
    let mut set = HashSet::new();
    split_apples(apples, plates, &mut curr, &mut set);
    println!("{}", set.len());
}

fn split_apples(apples: usize, plates: usize, prev: &mut Vec<usize>, set: &mut HashSet<Vec<usize>>) {
    if plates == 0 && apples == 0 {
        set.insert(prev.clone());
        return;
    }

    let idx = match prev.last() {
        Some(last) => *last,
        None => 0,
    };

    if plates == 1 {
        if apples >= idx {
            prev.push(apples);
            split_apples(0, plates - 1, prev, set);
            prev.pop();
        }
        return;
    }

    for i in idx..=apples {
        prev.push(i);
        split_apples(apples - i, plates - 1, prev, set);
        prev.pop();
    }
}

// #include <iostream>
// using namespace std;
// int f(int m,int n){
//     if(m<0 || n<0)
//         return 0;
//     else if(m==1 || n==1)
//         return 1;
//     else
//         return f(m,n-1)+f(m-n,n);
// }
// int main(){
//     int m,n;
//     while(cin >> m >> n){
//         cout << f(m,n) << endl;
//     }
//     return 0;
// }