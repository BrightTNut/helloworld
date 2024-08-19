fn main() {
    println!("Hello, world!");
//Borrowing and Refrences
let mut _x = 5;
let _r = &mut _x;
// for doing arthemadic opration
*_r += 2;
println!("Value of _x after referancing : {_r}");

//Example
let mut account = BankAccount{
    owner:"Prathmesh".to_string(),
    balance:1550.23,
};
//withdraw amount
account.withdraw(50.5);
//print out of balance
account.print_balance();

}

struct BankAccount{
    owner:String,
    balance:f64,
}
//struct and impl name shuld be same 
impl BankAccount{
    fn withdraw(&mut self,amount: f64){
        println!("Withrawing {} from {} account!!.",amount,self.owner);
        self.balance -= amount;
    }
    fn print_balance(&self){
        println!("Now Balance of Account is {}",self.balance);
    }
}