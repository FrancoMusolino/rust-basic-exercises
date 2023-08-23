// 2) For the code below defines a Person struct. In the main program, we created a vector that holds instances of Person and added several entries to it. Your task is to write a statement using iterators to collect all the ages of the different persons into a single vector.

use std::collections::HashMap;

struct Person {
    first_name: String,
    last_name: Option<String>,
    age: u8,
}

impl Person {
    fn new(first_name: String, last_name: Option<String>, age: u8) -> Self {
        Person {
            first_name,
            last_name,
            age,
        }
    }
}

fn main() {
    let mut persons: Vec<Person> = Vec::new();

    persons.push(Person::new(
        String::from("Juancito"),
        Some(String::from("Bostero")),
        20,
    ));

    persons.push(Person::new(
        String::from("Juan"),
        Some(String::from("Bostero")),
        18,
    ));

    persons.push(Person::new(String::from("Juanchito"), None, 18));

    persons.push(Person::new(String::from("Pepito"), None, 32));

    persons.push(Person::new(
        String::from("Pepe"),
        Some(String::from("Argento")),
        56,
    ));

    persons.push(Person::new(String::from("Nono"), None, 56));

    persons.push(Person::new(
        String::from("Pep"),
        Some(String::from("Guardiola")),
        56,
    ));

    let ages = persons.iter().map(|person| person.age).collect::<Vec<u8>>();

    let mut group_by_ages: HashMap<u8, i32> = HashMap::new();

    for age in ages {
        match group_by_ages.get(&age) {
            Some(_) => {
                group_by_ages.entry(age).and_modify(|count| *count += 1);
                (())
            }
            None => {
                group_by_ages.insert(age, 1);
                (())
            }
        }
    }

    println!(
        "Un total de {} edades distintas registradas",
        group_by_ages.len()
    );
    println!("Las edades registradas son: {:?}", group_by_ages)
}
