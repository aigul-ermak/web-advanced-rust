
// Struct
struct User {
    username: String,
    email:String,
    sign_in_count: u64,
    active: bool
}

//Initialize, User with attributes
fn initialize_struct(){
    let user1 = User {
        email:String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
}

// when params have the same name you can just pass them
fn initialise_struct_simple(email: String, username: String){
 let user1 = User{
    // email: email,
    // username:username,
    email,
    username,
    active: true,
    sign_in_count: 1,
 };
}

// updating value
fn update_struct(){
    let mut user1 = User{    
    email:String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

    user1.email = String::from("some_new@exapmle.com");
    //inherit the rest of the values from one user
    let user2 = User {
        email:String::from("someone_email@example.com"),
        username: String::from("some_username1234"),
        ..user1
    };
}

// Methods
 struct Person {
    first_name: String,
    last_name: String,

 }

 impl Person {
    // method (uses "&self")
    fn details(&self) -> String {
        String::from(&self.last_name)
    }

    // associated function (does not use "&self")
    fn more_details() -> String {
        String::from("pizza")
    }
 }


 //
 fn example() {
    let george = Person {
        first_name: String::from("George"),
        last_name: String::from("Lopez"),
    };
    println!("{}", george.details());
    println!("{}", Person::more_details());
 }


fn main() {
    println!("Hello, world!");
}
