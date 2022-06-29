use std::net::IpAddr;

fn main() {
    let mut mask = String::new();
    let mut ipaddr1 = String::new();
    let mut ipaddr2 = String::new();

    std::io::stdin()
        .read_line(&mut mask).unwrap();
    std::io::stdin()
        .read_line(&mut ipaddr1).unwrap();
    std::io::stdin()
        .read_line(&mut ipaddr2).unwrap();
    let nmask = match ip_to_number(mask) {
        Some(n) => n,
        None => {
            println!("{}", 1);
            return;
        }
    };
    let nip1 = match ip_to_number(ipaddr1) {
        Some(n) => n,
        None => {
            println!("{}", 1);
            return;
        }
    };
    let nip2 = match ip_to_number(ipaddr2) {
        Some(n) => n,
        None => {
            println!("{}", 1);
            return;
        }
    };
    if ((nmask - 1) & !nmask) != !nmask {
        println!("{}", 1);
        return;
    }
    if nip1 & nmask == nip2 & nmask {
        println!("0");
    } else {
        println!("2");
    }
}

fn ip_to_number(ip: String) -> Option<u32> {
    let mut ret = 0u32;
    let mut ok = true;
    ret = ip.trim().split(".").map(|s| s.parse::<i32>().unwrap()).fold(0, |mut ret, num| {
        if num < 0 || num >= 256 {
            ok = false;
        }
        ret <<= 8;
        if num >= 256 {
            ok = false;
        }
        ret += num as u32;
        ret
    });
    if ok {
        Some(ret)
    } else {
        None
    }
    
}