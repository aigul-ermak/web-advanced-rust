
fn tuples() {

    let string_tuple = ("milk", "fish", "salad", "rice");

    let num_tuple = (1, 2, 3, 4, 5);

    let mixed_tuple = ("Sam", 1, 5.6, "A");

    // for `char` literal, use single quotes
    let pair: (char, i32) = ('a', 17);

    println!("{}", string_tuple.0);

    println!("{}", num_tuple.2);

    println!("{}, {}", mixed_tuple.0, mixed_tuple.1);

    println!("{}, {}", pair.0, pair.1);
}


fn main() {   
    tuples()
}
