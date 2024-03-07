enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn readIpAddress(ip: IpAddress) -> u8 {
    match ip {
        IpAddress::V4(_, _, _, _) => 0,
        IpAddress::V6(_) => 1,
    }    

}

fn main() {
    println!("Hello, world!");
    let home = IpAddress::V4(127, 0, 0, 1);
}
