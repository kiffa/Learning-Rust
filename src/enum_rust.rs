// mod guess_number;

// use std::io;
#[derive(Debug)]
enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

//
// struct IpAddr {
//     kind: IpAddressKind,
//     address: String,
// }


fn route(ip_kind: IpAddressKind) {}

fn main() {
    println!("Hello enum!");


    let home = IpAddressKind::V4(127, 0, 0, 1);

    let loopback = IpAddressKind::V6(String::from("::1"));

    println!("Home IP config: {:?}", home);
    //
    // println!("Home IP config: {:?}", loopback);
}


