fn main() {
    println!("Hello, world!");

    //compound data types
    //array, tuples ,slices and strings

//Array
let numbers: [i8;5]= [1,2,3,4,5];
println!("Array of nubers:{:?}",numbers);
let fruits : [&str;3]=["Apple","Banana","Orange"];
println!("Fruits Array:{:?}",fruits);
println!("Fruits Array 0:{}",fruits[0]);
println!("Fruits Array 1:{}",fruits[1]);
println!("Fruits Array 2:{}",fruits[2]);

//TRUPLES
let humans:(String,i32,bool) =("Alice".to_string(),90,true);
println!("Humanes truple:{:?}",humans);

let mixtruple =("Kratos",89,true,[1,2,3,4,5]);
print!("Mix Truple:{:?}",mixtruple);

//Slices :[1,2,3,4,5]
let numbers_slices:&[i32]=&[1,2,3,4,5];
println!("numbers Slices:{:?}",numbers_slices);

let fruits_slices:&[&str]=&["Apple","Banana","Mango"];
println!("Fruits Slices:{:?}",fruits_slices);

let animals_slices:&[&String]=&[&"Dog".to_string(),&"cat".to_string(),&"lion".to_string()];
print!("animals Slices: {:?}\n",animals_slices);

//Strings vs String Slices (&str)
//Strings= growable,mutable,owned string type

//mutable string variable
let mut stone_cold:String = String::from("Hello, ");
stone_cold.push_str("Yeah!..");
println!("Stone Cold Syas: {}",stone_cold);


//&str (string slice): it is refrance of string not original string
let string:String=String::from("Helllo, World!");
print!("String value : {}",string);
let slice :&str = &string[0..5];
print!("\nSlices String : {}",slice);
}