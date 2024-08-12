fn main() {
    println!("Hello, world!");

    //function and / variavle should be written in snake case
    //snake case : name_with_underline = hello_world
    //kebab case : hello-world

    hello_world();
    tell_height(180);
    human_id("Tejas".to_string(),21, 180.7);

    let x : i8= {
        let price : i8 = 5;
        let qty : i8 = 10;
        //default return is present at the end of expression in Rust
        price * qty
    };
    print!("\nResult of Expression : {}",x);
    //add function
    print!("\n Result of 'add' Function: {}." , add(7,7));

    //bmi function call
    let weight : f64 = 64.7;
    let height : f64 = 1.80;
    let bmi : f64 = calculate_bmi(weight, height);
    //{:.2} for allow second decimal after point 
    print!("BMI of your Body : {:.2}. ",bmi);
}

//Hoisiting - calling function anywhere in the code
fn hello_world(){
    print!("In Hello World Function !!!\n")
}

fn tell_height(height: u8){
    print!("Your Height is : {} cm",height);
}

fn human_id(name :String,age:u8,height:f32){
    print!("\nMy name is {}, My age is {}, My height is {}",name,age,height);
}

fn add(a:i8,b:i8)-> i8{
    return a + b;
}
//Expression and Statments
//Expression : Anything that returns Value
//Statments : Anything that not return any value

//statements
//almost all statements in Rust end with ; .
//let y = let x= 10;
//1 variable decrations : let x =5;
//2 function definitions : fn foo(){}
//3 control flow statements (loops)

//final example 
//BMI = kg/cm*cm

fn calculate_bmi(weight_kg: f64,height_m:f64)-> f64{
     weight_kg / (height_m * height_m)
}