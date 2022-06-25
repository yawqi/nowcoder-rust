use std::num::ParseIntError;

fn main() {
    let mut a_count = 0;
    let mut b_count = 0;
    let mut c_count = 0;
    let mut d_count = 0;
    let mut e_count = 0;
    let mut error_count = 0;
    let mut private_count = 0;
    loop {
        let mut s = String::new();
        let bytes = std::io::stdin().read_line(&mut s).unwrap();
        if bytes == 0 {
            break;
        }
        let ips: Vec<_> = s.trim().split('~').collect();
        let ip = ips[0].parse::<IpAddr>();
        if ip.is_err() {
            error_count += 1;
            continue;
        }
        let ip = ip.unwrap();
        match ip.address_type {
            AddressType::NotCount => continue,
            _ => (),
        }
        let mask = ips[1].parse::<IpAddr>();
        if let Ok(mask) = mask {
            if !mask.is_valid_mask() {
                error_count += 1;
                continue;
            }
        } else {
            error_count += 1;
            continue;
        }
        match ip.address_type {
            AddressType::A => a_count += 1,
            AddressType::B => b_count += 1,
            AddressType::C => c_count += 1,
            AddressType::D => d_count += 1,
            AddressType::E => e_count += 1,
            AddressType::UnKnown => error_count += 1,
            AddressType::NotCount => continue, 
        }
        if ip.is_private() {
            private_count += 1;
        }
    }
    println!("{} {} {} {} {} {} {}", a_count, b_count, c_count, d_count, e_count, error_count, private_count);
}

struct IpAddr {
    A: u8,
    B: u8,
    C: u8,
    D: u8,
    pub address_type: AddressType,
}

enum AddressType {
    A,
    B,
    C,
    D,
    E,
    UnKnown,
    NotCount,
}

impl std::str::FromStr for IpAddr {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<_> = s.split('.').collect();
        if v.len() != 4 {
            return Err(());
        }
        let mut v: Vec<_> = v.into_iter().map(|s| {
            s.parse::<u8>()
        }).collect();
        let mut result = vec![];
        for re in v {
            if re.is_err() {
                return Err(());
            }
            result.push(re.unwrap());
        }
        let address_type = match result[0] {
            1..=126 => AddressType::A,
            128..=191 => AddressType::B,
            192..=223 => AddressType::C,
            224..=239 => AddressType::D,
            240..=255 => AddressType::E,
            0 | 127 => AddressType::NotCount,
            _ => AddressType::UnKnown,
        };
        Ok(
            IpAddr {
                A: result[0],
                B: result[1],
                C: result[2],
                D: result[3],
                address_type,
            }
        )
    }
}

impl IpAddr {
    fn is_valid_mask(&self) -> bool {
        if self.A == 0 && self.B == 0 && self.C == 0 && self.D == 0 {
            return false;
        };
        if self.A == 255 && self.B == 255 && self.C == 255 && self.D == 255 {
            return false;
        }
        let mask: u32 = (self.A as u32) << 24 | (self.B as u32) << 16 | (self.C as u32) << 8 | self.D as u32;
        ((mask - 1) & !mask) == !mask
    }

    fn is_private(&self) -> bool {
        self.A == 10
            || (self.A == 172 && self.B >= 16 && self.B <= 31)
            || (self.A == 192 && self.B == 168)
    }
}