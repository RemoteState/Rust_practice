use std::collections::HashMap;

#[derive(PartialEq, Debug, Hash, Eq)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}

fn main() {
    let vikings = HashMap::from([
        (Viking::new("Eivor", "Norway"), 25),
        (Viking::new("Eitr", "England"), 30),
        (Viking::new("ivotr", "Nor"), 65),
    ]);
    
    for (viking, health) in &vikings {
        println!("{:?} has {} hp", viking, health);
    }
}