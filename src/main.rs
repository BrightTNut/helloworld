
//For allowing warnings
#![allow(warnings)]

// Error is :`Person` doesn't implement `Debug`
// the trait `Debug` is not implemented for `Person`
// add `#[derive(Debug)]` to `Person` or manually `impl Debug for Person`
#[derive(Debug)]
enum GenderCato {
    Male,
    Female,
    Others
}

#[derive(Debug)]
struct Person {
    name:String,
    email:String,
    gender:GenderCato
}

fn main() {
    let per1 = Person{
        name:String::from("Pankaj"),
        email:String::from("kjhkjh@gmail.com"),
        gender:GenderCato::Male
    };
 println!("Person 1 : {:?}",per1);
 println!("Option enum::");
 let result = cal(4);

 println!("Number pass in cal fn is even or not : {:?}",result)
}


//Option enum
// syntax :
// enum Option<Type>{
//     Some{Type}
//     None
// }

//example of option enum using function whci find even no
fn cal(no:i32) -> Option<bool>{
    if(no%2 == 0){
        Some(true)
    }else {
        None
    }
}