fn main() {
    println!("Hello, world!");

    //Ownership 
    //Ownership : every value has a single owner 
    //every variable has one value, and it is its sole owner
    //Ownership intreduce in Rust to resolve memory safety issues and performance at same time
    
//     Ownership Rules
//     Each value in Rust has an owner.
//     There can only be one owner at a time.
//     When the owner goes out of scope, the value will be dropped.

    //example on each rule of ownership
    //1  Each value in Rust has an owner.
    let s1= String ::from("Rust");
    let len  = calculate_len(&s1);
    print!("Length of {s1} is : {len}");

//  2.   There can only be one owner at a time.
    print!("\nBefor :owner of {s1} is 's1' now");
    let s2 = s1;
    print!("\nAfter :owner of {s2} is 's2' now");

}

//  3.   When the owner goes out of scope, the value will be dropped.
fn print_out_side(){
    print!("Out side of scope value of s2 is {s1}");
}

fn calculate_len(s: &String)->usize{
  s.len()
}
    