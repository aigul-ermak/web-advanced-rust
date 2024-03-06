
enum OrderSource {
    Phone,
    Mobile,
    Email,
}

impl OrderSource {    
    fn from_str(input: &str) -> Option<OrderSource> {
        match input.to_lowercase().as_str() {
            "phone" => Some(OrderSource::Phone),
            "mobile" => Some(OrderSource::Mobile),
            "email" => Some(OrderSource::Email),
            _ => None,
        }
    }
}


// TODO #1: Define the Displayable trait.
// This trait should include a display method for implementing objects to provide a custom display functionality.
trait Displayable {
    //
}


struct Order {
    name: String,
    year: u32,
    source: OrderSource,
    item_number: u32,
    count: u32,
}


// TODO #2: Implement the `display` method for the `Order` struct as part of the `Displayable` trait.
// This method should neatly format and print the details of an order, including its name, year, source, item number, and count.
impl Displayable for Order {
    fn display(&self) {

       

    }
}


struct Delivery {
    destination: String,
    day: u32,
    source: OrderSource,
    weight: f32,
}

// TODO #3: Implement the `display` method for the `Delivery` struct within the `Displayable` trait.
// This method should output a formatted string detailing the delivery's destination, day, source, and weight.
impl Displayable for Delivery {
    fn display(&self) {
        


    }
}



fn main() {   
  let order = Order {
        name: "Pizza".to_string(),
        year: 2023,
        source: OrderSource::Email,
        item_number: 1,
        count: 2,
    };

    let delivery = Delivery {
        destination: "123 Main St".to_string(),
        day: 5,
        source: OrderSource::Mobile,
        weight: 5.0,
    };
   
    order.display();
    delivery.display();
}
