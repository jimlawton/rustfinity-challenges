pub enum Animal {
    Dog,
    Cat(String),
    Bird {species: String, can_fly: bool},
}

pub fn describe_animal(animal: &Animal) -> String {
    match animal {
        Animal::Dog => "A friendly dog.".to_string(),
        Animal::Cat(name) => format!("A cat named {name}."),
        Animal::Bird {species, can_fly} => {
            if *can_fly == true {
                format!("A {species} that can fly.")
            } else {
                format!("A {species} that cannot fly.")
            }
        }
    }
}

// Example use case
pub fn main() {
    let dog = Animal::Dog;
    assert_eq!(describe_animal(&dog), "A friendly dog.");

    let cat = Animal::Cat("Whiskers".to_string());
    assert_eq!(describe_animal(&cat), "A cat named Whiskers.");

    let bird = Animal::Bird {
        species: "Penguin".to_string(),
        can_fly: false,
    };
    assert_eq!(describe_animal(&bird), "A Penguin that cannot fly.");
}

