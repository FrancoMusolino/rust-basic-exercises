// 3) You are tasked with implementing a library management system using Rust. Your goal is to design a program that can handle books and magazines. To fulfill the requirements, follow the steps below:

// Create a structure called Item with the following fields:

// id: An integer representing the unique identifier of the item.

// title: A string representing the title of the item.

// year: An integer representing the publication year of the item.

// type: an enumeration type. The details are coming below.

// Create an enumeration called ItemType with two variants:

// Book: Represents a book.

// Magazine: Represents a magazine.

// Implement a function called display_item_info() that takes an Item as input and displays its information. The function should output the item's ID, title, publication year, and type (book or magazine).

use nanoid::nanoid;

#[derive(PartialEq)]
enum ItemType {
    BOOK,
    MAGAZINE,
}

struct Item {
    id: String,
    title: String,
    year: i32,
    item_type: ItemType,
}

impl Item {
    fn new(title: String, year: i32, item_type: ItemType) -> Self {
        Item {
            id: nanoid!(),
            title,
            year,
            item_type,
        }
    }
}

fn display_item_info(item: &Item) {
    let item_type: &str;
    if item.item_type == ItemType::BOOK {
        item_type = "El libro";
    } else {
        item_type = "La revista";
    }

    println!("INTERNO: id => {}", item.id);
    println!(
        "{} '{}', ha sido publicado en el {}",
        item_type, item.title, item.year
    );
}

fn main() {
    let mut library: Vec<Item> = Vec::new();

    library.push(Item::new(
        String::from("La remontada"),
        2016,
        ItemType::BOOK,
    ));
    library.push(Item::new(
        String::from("Campeones del mundo"),
        2022,
        ItemType::MAGAZINE,
    ));
    library.push(Item::new(
        String::from("Messi, y solo Messi"),
        2023,
        ItemType::BOOK,
    ));

    for item in library.iter() {
        display_item_info(item)
    }
}
