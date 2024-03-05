fn main() {

//ANCHOR - 1
   let num = 10;
   println!("the number is {}", num);

//ANCHOR - 2
let name: &str = "Sam";
let last_name: &str = "Paul";
println!("His name is {} {}", name, last_name);

//ANCHOR - 3 change variable from immutable to mutable
// error
//num = 100;
let mut num = 100;
println!("the number is {}", num);

}
