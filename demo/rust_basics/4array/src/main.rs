//ANCHOR - arrays

fn arrays() {
    // Mutatable

    let mut arr: [i32; 3] = [4, 5, 3];

    // change value if array item
    arr[0] = 0;
    arr[1] = 2;

    for x in &arr {
        println!("{}", x);
    }

    // initialize array with expression
    let exp_array: [i32; 5] = [0; 5];
    // if the same value we can use it
     
     for x in exp_array {
        println!("{}", x);
    }

    // Initialize variables with an array
    let [greg, mark] = ["Greg".to_string(), "Mark".to_string()];
    println!("{}", greg);
    println!("{}", mark);

}

fn main() {  

   arrays();      

}
