enum IpAdresse {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let x = IpAdresse::V4(127, 0, 0, 1);
    let y = IpAdresse::V6(String::from("::1"));

    route(x);
    route(y);
}

fn route(ip : IpAdresse) {  }
