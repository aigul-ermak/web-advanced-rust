
//ANCHOR - if else
fn if_else_conditional() {
    let number = 3;

    if number < 5 {
        println!("True!")
    } else if number == 5 {
        println!("Ok!")
    } else {
        println!("False")
    }
}


//ANCHOR - Loops
// infinite loop
fn infinite_loop() {
    loop {
        println!("Infinite loop");
    }
}

// for loop
fn for_loop() {
    for n in 1..40 {
         println!("The price is ${} ", n);
    }
}

fn loop_with_continue() {
    let mut n = 0;

    
    loop{ 
        n += 1;
        if n == 13 {
            continue; 
        }        
        
        if n == 20 {
            break;
        }

        println!("The value of n is {}", n);
    }
}

fn for_in_range () {

    for i in 1..11 {
        println!("Number is {}", i);
    }
}


fn while_loop() {
    let mut n = 0;
    
    while n < 30 { 
        n += 1;

        if n == 20 {
            continue; 
        }     
               
        if n == 29 {
            break;
        }

         println!("{}", n);
    }
}

fn match_number() {
    let x = 5;

    match x {
        0 => {
            println!("found zero");
        }
        // multiple values
        1 | 2 => {
            println!("found 1 or 2");
        }
        // range
        3..=9  => {
            println!("found 3 to 9");
        }
        _ => {
             println!("found something else");
        }
    }
}


fn main() {
    //if_else_conditional();

    //infinite_loop();

    //for_loop();

    //loop_with_continue();

    //for_in_range();

    //while_loop();

    match_number();
}
