
//For allowing warnings
#![allow(warnings)]

fn main() {
    println!("Hello, world!");
//Repetition with loops:loop,while, for
//Doing things over and over
//loop : it will run untill we apply break to it
loop{
    println!("Hello Tejas");
    println!("Hello Tejas");
break;
}
//example
let mut counter = 0;
let result = loop{
    counter +=1;
    if counter == 10{
        break counter *7;
    }
};
println!("Final value of Counter : {result}");

//Loop labels to disambiguate between multiple loops
//ex
let mut count = 0;
'counting_up: loop{
     println!("count : {count}");
     let mut remaining = 10;
     loop{
        println!("Remaining : {remaining}");
        if remaining == 9{
            break;
        }
        if count == 2{
            break 'counting_up;
        }
        remaining -=1;
     }
     count +=1;
    }

    //while loop
    let mut number = 5;
    while number !=0{
        println!("Number :{number}");
        number-=1;
    }

//Looping through collection with loop
 let a =[1,2,3,4,5,6,7];
 for element in a{
    print!("{element} ")
 }
 let b =["a","b","c"];
 for letter in b{
    print!(" {letter}")
 }
}
