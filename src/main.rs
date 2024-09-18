
//For allowing warnings
#![allow(warnings)]

fn main() {
    println!("Hello, world!");
//Error Handling Approaches
//Approach 1
enum Option<T>{// Define the generic Option type
    Some(T), // Represents a value
    None, // represents no value
}
//example is on line 28

match divide(10.0,2.0){
    Some(x) => println!("Approach 1 Result : {}",x),
    None => println!("Cannot divide by 0!"),
}

//Approach 2 : it is like try and catch in javascript
enum Result<T,E>{//Define the generic Result type
    Ok(T),//Represent a value
    Err(E),//Respresent Error
}

//example is on line 45

match divide_two(100.23,0.0) {
    Ok(x) => println!("Approach 2 Result :{}",x),
    Err(err) => println!("Error: {}",err),
}

}
//time : 2.35.00
//exampleon Approach 1
fn divide(numerator:f64,denominator:f64)->Option<f64>{
    if denominator == 0.0{
        None
    }else{
        Some(numerator / denominator)
    }
}

//Example for Approach 2
fn divide_two(numerator:f64,denominator:f64)->Result<f64,String>{
    if denominator == 0.0 {
        Err("Cannot Divide By 0 !!".to_string())
    }else{
        Ok(numerator/denominator)
    }
}