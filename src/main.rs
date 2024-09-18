
//For allowing warnings
#![allow(warnings)]

fn main() {
    println!("Hello, world!");
//Enum                         
enum IpAddrKind{
    V4,V6
}

let _four = IpAddrKind::V4;
let _six = IpAddrKind::V6;

//use in functions
fn route(_ip_kind:IpAddrKind){}
route(IpAddrKind::V4);
route(IpAddrKind::V6);

//Using struct
struct IpAddr{
    kind: IpAddrKind,
    address:String
}

let home = IpAddr{
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1")
};

let loopback = IpAddr{
    kind: IpAddrKind::V6,
    address: String::from("::1")
};

//Enum is not understandable so read documentation for this
}
