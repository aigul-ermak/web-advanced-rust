
trait Speak {
    fn speak(&self);
}

struct Dog {
    name: String,
}

struct Robot {
    name: i32,
}

impl Speak for Dog {
    fn speak(&self) {
        println!("{} says: Woof!", self.name);
    }
}

impl Speak for Robot {
    fn speak(&self) {
        println!("Model {} says: Beep boop!", self.name);
    }
}


fn main() {
    let dog = Dog { name: "Buddy".to_string() };
    let robot = Robot { name: 101 };

    
    dog.speak();
    robot.speak();
}
