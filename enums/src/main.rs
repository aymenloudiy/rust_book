enum IpAddrKind {
    V4,
    V6,
}
fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    let m = Message::Write(());
    m.call();
}
fn route(_ip_kind: IpAddrKind) {}
// worse way of doing it
fn _ex1() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
// better way of doing it
fn _ex2() {
    enum IpAddr2 {
        V4(String),
        V6(String),
    }

    let _home2 = IpAddr2::V4(String::from("127.0.0.1"));

    let _loopback2 = IpAddr2::V6(String::from("::1"));
}
fn _ex3() {
    // even better way of doing it
    struct Ipv4Addr2 {
        // whatever struct you want
    }

    struct Ipv6Addr2 {
        // whatever struct you want
    }

    enum IpAddr3 {
        V4(Ipv4Addr2),
        V6(Ipv6Addr2),
    }
}
// example
enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(()),
    _ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
fn _opt() {
    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;
}
