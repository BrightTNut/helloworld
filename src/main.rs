fn main() {
    println!("Hello, world!");
//Shadowing
let x=9;
let x=x+5;
{
     let x= x *3;
    println!("The value of x in inner : {}",x);

}
println!("Value of x is:{}",x);

}
