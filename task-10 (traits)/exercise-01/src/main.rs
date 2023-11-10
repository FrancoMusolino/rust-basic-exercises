// You are working on a project to develop a simulation game where players can own and trade virtual pets. Each pet has a unique ID and a set of characteristics. Your task is to implement the necessary traits and structures to model the pets and their interactions.

// Task 1. Define a structure Dog with the following fields:

// · id: A unique ID of type u32 representing the dog's ID.
// · name: A string representing the dog's name.
// · age: An integer representing the dog's age.

// Task 2. Implement the Dog with the following method:

// Method: fn new(id: u32, name: String, age: u32) -> Self – Creates and returns a new instance of the Dog.

// Task 3. Define a trait called Trade with the following associated types and methods:

// · Associated Type: Pet - Represents the type of a pet being traded. Currently, we have only a dog as a pet but later on we may add more animals as pets.

// · Method: fn trade(&self, other: &Self::Pet) -> bool - Performs a trade with another pet and returns true if the trade is successful, false otherwise.

// Task 4. Implement the trait Trade for the Dog structure with the following method:

// · Method: fn trade(&self, other: &Self::Pet) -> bool - Implements the trading logic between two dogs. We will use a simple logic. The trade will only be successful if self.id > other.id. Please note that the Trade implementation for Dog will set the associated type of Pet to that of Dog.

// Task 5. Define a trait called Playable which is a super trait of Trade. the implementation is empty though and may be later on modified.

// Task 6. Implement the trait Playable for the Dog. The implementation will be empty which means that it will serve as a marker trait (meaning that the type Dog has this property).

// Task 7. Your task is to implement the above traits and structures with appropriate trait bounds, associated types, and method implementations. Additionally, provide an example of how the Dog structure can be instantiated and demonstrate the usage of the trade method for trading two dogs.

use rand::random;

trait Playable {}

trait Trade: Playable {
    type Pet;
    fn trade(&self, other: &Self::Pet) -> bool;
}

#[derive(Debug)]
struct Dog {
    id: u32,
    name: String,
    age: u8,
}

impl Dog {
    fn new(name: String, age: u8) -> Self {
        Self {
            id: random::<u32>(),
            name,
            age,
        }
    }
}

impl Trade for Dog {
    type Pet = Dog;

    fn trade(&self, other: &Self::Pet) -> bool {
        self.id > other.id
    }
}

impl Playable for Dog {}

fn main() {
    let first_dog = Dog::new("Pepe".to_string(), 4);
    let second_dog = Dog::new("Juancito".to_string(), 6);

    println!(
        "Trading between dog {} ({}) and dog {} ({})",
        first_dog.name, first_dog.age, second_dog.name, second_dog.age
    );

    let successful_trade = first_dog.trade(&second_dog);

    if successful_trade {
        println!("Intercambio realizado con éxito");
    } else {
        println!("No se pudo realizar el intercambio");
    }
}
