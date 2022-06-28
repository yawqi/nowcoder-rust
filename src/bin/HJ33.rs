use std::net::IpAddr;

fn main() {
    let mut ip_addr = String::new();
    let mut ip_num = String::new();
    std::io::stdin()
        .read_line(&mut ip_addr);
    std::io::stdin()
        .read_line(&mut ip_num);

    let num_ret = ip_addr.trim().split(".").map(|n| n.parse::<u32>().unwrap()).fold(0, |mut a, n| {
        a<<=8;
        a+=n;
        a
    });

    let mut number = ip_num.trim().parse::<u32>().unwrap();
    let mut nums = vec![];
    while number != 0 {
        let n = number % 256;
        number >>= 8;
        nums.push(format!("{}", n));
    }
    nums.reverse();
    let s_ret = nums.join(".");
    println!("{}", num_ret);
    println!("{}", s_ret);
}