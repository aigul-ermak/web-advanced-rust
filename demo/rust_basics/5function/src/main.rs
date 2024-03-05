
//ANCHOR - function that return a 32-bit signed integer
fn fn_return_value() -> i32 {
    4
}

//ANCHOR - fn to lowercase
fn fn_lower_case() -> String {
    let s = "DOCTOR";
    let s_lower = s.to_ascii_lowercase();
    return s_lower;
}

//ANCHOR - omit ; and return word
fn give_me_string() -> String {
   // return String:: from ("pizza");

   //we can omit return word and removed semicolon
    String:: from ("pizza")
}


fn main() {

    //println!("{}", fn_return_value);    

    //println!("{}", fn_lower_case());

    //println!("{}", give_me_string());    
    
}
