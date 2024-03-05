
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

trait Displayable {
    fn display(&self);
}


struct Order {
    name: String,
    year: u32,
    source: OrderSource,
    item_number: u32,
    count: u32,
}

impl Displayable for Order {
    fn display(&self) {
        let source = match self.source {
            OrderSource::Phone => "Phone",
            OrderSource::Mobile => "Mobile",
            OrderSource::Email => "Email",
        };
        println!(
            "Order for {}, year: {}, made by: {}, item number: {}, count: {}",
            self.name, self.year, source, self.item_number, self.count
        );
    }
}

struct Delivery {
    destination: String,
    day: u32,
    source: OrderSource,
    weight: f32,
}

impl Displayable for Delivery {
    fn display(&self) {
        let source = match self.source {
            OrderSource::Phone => "Phone",
            OrderSource::Mobile => "Mobile",
            OrderSource::Email => "Email",
        };
        println!(
            "Delivery to {}, day: {}, arranged via: {}, weight: {}kg",
            self.destination, self.day, source, self.weight
        );
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
