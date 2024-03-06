use std::collections::HashMap;

//ANCHOR - HashMap
fn hash_map() {
  let mut contacts = HashMap::new();
    contacts.insert("Daniel", "798-1364");
    contacts.insert("Ashley", "645-7689");

    match contacts.get(&"Daniel") {
        Some(&number) => println!("Calling Daniel: {}", number),
        _ => println!("Don't have Daniel's number."),
    }

    contacts.insert("Daniel", "164-6743");

    match contacts.get(&"Ashley") {
        Some(&number) => println!("Calling Ashley: {}", number),
        _ => println!("Don't have Ashley's number."),
    }

    contacts.remove(&"Ashley");

    for (contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, number);
    }
}


fn main() {     
 hash_map();
}
