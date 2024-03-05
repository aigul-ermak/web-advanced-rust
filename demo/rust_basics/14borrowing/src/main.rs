

fn main() {
    let s1 = "hello".to_string();   // s1 is owner "hello"
    let s2 = &s1;                   // s2 borrow s1
     
    println!("s1: {}", s1);     // s1: hello
    println!("s2: {}", s2);     // s2: hello


    // let x = 42;
    // let x_ref1 = &x;
    // let x_ref2 = &x;
    // let x_ref3 = &x;
    // println!("{} {} {}", x_ref1, x_ref2, x_ref3);
 
}