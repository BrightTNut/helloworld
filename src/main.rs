
//For allowing warnings
#![allow(warnings)]

fn main() {
    println!("Hello, world!");
//Loops control flow
// if and else have same data type of output must be return
// if..else
let age:u8 = 19;
if age>=18{
 println!("You can drive a Class....!")
}else{
    println!("You can not drive a Class....!")
}

//Multiple conditions with  if..else  
let number = 11;
if number % 2 == 0{
    println!("Number is Even.!");
}else if number % 3 == 0{
    println!("Number is Odd.!");
}else {
    println!("Number is Prime.!");
}

//using let in statments
let condition = true;
let numbers = if condition {5} else {6};
println!("Number is {numbers}.!");

}
