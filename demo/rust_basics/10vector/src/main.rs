use std::collections::HashMap;

//ANCHOR vectors
fn vectors() {

    let vector: Vec<i32> = vec![1, 3, 7, -1];
    
    let mut vector : Vec<i32> = (0..10).collect();
 
    // for x in &vector {
    //     println!("{}", x);
    // }   

    // Append
    vector.push(-2);
    println!("{}", vector[vector.len() - 1]);

    // remove value
    // println!("{}", vector.pop()); // Error - return Option<i32> 
    println!("{:?}", vector.pop()); // remove last value
     for x in &vector {
        println!("{}", x);
    }   
}

// Iterating
fn iterating() {   

    // Vectors = Iterate the same as Arrays
    let vector: Vec<i32> = (0..5).collect();
    for x in vector.iter() {
        println!("{}", x);
    }

    let mut mut_vector: Vec<i32>= (0..5).collect();
    for x in mut_vector.iter_mut() {
        // dereferencing
        *x += 3;
    }
    println!("{:?}", mut_vector);
}



fn main() {    
// vectors();

// iterating();
 
}
