use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }
    fn greet_visitor(&self) {
        println!("{} {}, welcome!", self.greeting, self.name);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name
        .trim()
        .to_lowercase()
}

fn main() {
    let visitor_list = [
        Visitor::new("John", "Hello"),
        Visitor::new("Mary", "Good morning"),
        Visitor::new("Joe", "Good evening"),
    ];

    println!("Hello, what's your name?");
    let name = what_is_your_name();
    let mut allow_them_in = false;
    for visitor in &visitor_list {
        if visitor.name == name {
            allow_them_in = true;
            visitor.greet_visitor();
        }
    }
    if !allow_them_in {
        println!("Sorry, you aren't on the list.");
    }
}
