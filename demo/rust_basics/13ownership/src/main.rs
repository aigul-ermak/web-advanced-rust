

//ANCHOR 
fn main() {
    let s1 = "hello".to_string();   // s1 is owner "hello"
    println!("s1: {}", s1);
     
    let s2 = s1;                    // value moved here to  s2
 
    //println!("s1: {}", s1);       // Error
    println!("s2: {}", s2);

    // new value for s1
    let s1 = "bye".to_string();
    println!("s1: {}", s1);
 
}

