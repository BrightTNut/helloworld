
//For allowing warnings
#![allow(warnings)]


enum CarTypes{
    Hatchback,Seden,SUV,MUV
}
fn printcars(car:CarTypes){
    match car{
        CarTypes::Hatchback =>{
            println!("Hatch car is beast!!")
        }
        CarTypes::Seden =>{
            println!("Seden car is beast!!")
        }
        CarTypes::SUV =>{
            println!("SUV car is beast!!")
        }
        CarTypes::MUV =>{
            println!("MUV cae is beast!!")
        }
    }
}
fn main() {
    println!("Hello, world!");
//calling function
printcars(CarTypes::Hatchback);

}