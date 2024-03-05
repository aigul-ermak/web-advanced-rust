//ANCHOR - strings
fn strings() {
    // create new empty immutable string
   let my_empty_string: String = String::new();

   // create string witn content
   //let my_string : String = String::from("Hello");
   //for update
   let mut my_string : String = String::from("Hello, ");
   println!("{}", my_string);

    // update string
   my_string.push_str("wmad sen B");
   println!("{}", my_string);

   // get &str from a String, borrow it
   let slice: &str = &my_string;
   println!("{}", slice);

    // try to modify the slice - error
    //slice.push_str("bye");
    //println!("{}", slice);

    // mut str
    let slice_mut:&mut str = &mut my_string;
    println!("{}", slice_mut);

}

fn str_string() {
    // 
    let word : String = String::from("Hello, World");
    
    let slice = &word[0..5];
    println!("{}", word);
    println!("{}", slice);
}


//ANCHOR - String iterating
fn iterating_over_string(){
    let str = String::from("Hello");
    
    // Loop through each character in a string using chars() method
    for char in str.chars() {
        println!("{}", char);
    }
}

fn main() {
  // strings();

  // str_string();

  //iterating_over_string();
}
