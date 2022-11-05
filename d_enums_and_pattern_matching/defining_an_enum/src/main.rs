enum MyIpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main() {
    let _home = MyIpAddr::V4(127, 0, 0, 1);
    let _loopback = MyIpAddr::V6(String::from("::1"));
}
