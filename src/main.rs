
//For allowing warnings
#![allow(warnings)]

fn main() {
    println!("Hello, world!");
//struct
struct Book{
    title: String,
    author: String,
    pages: u32,
    available: bool
}

struct User{
    active : bool,
    username: String,
    email: String,
    sign_in_count: u64
}

//user1 is instance 
let mut user1= User{
    active: true,
    username:String::from("Someone"),
    email:String::from("Someone@gmail.com") ,
    sign_in_count:1,
};

user1.email=String::from("another@gmail.com");
println!("Email is of User1:{}",user1.email);

//Return a struct from function
fn build_user(email:String,username:String)->User{
    User { active: true, username, email, sign_in_count: 1 }
}

//create instances from other instance
let user2 =User{
    ..user1
};
println!("User2 data which from user1: {} , {}",user2.username,user2.email);

//Tuple Structs
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

let black=Color(0,0,0);
let white=Color(255,255,255);


//Unit-like struct
struct AlwayseEquals;
let subjet=AlwayseEquals;
}
