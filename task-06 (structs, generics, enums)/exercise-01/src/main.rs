// 1) Consider the program below. Modify it to eliminate the usage of all the let statements, both inside the main function and the other functions.

// struct Fruit {
//     apples: i32,
//     bananas: i32,
// }

// fn increase_fruit(fruit: Fruit) -> Fruit {
//     let fruit = Fruit {
//         apples: fruit.apples * 2,
//         bananas: fruit.bananas * 3,
//     };
//     fruit
// }

// fn new_fruit() -> Fruit {
//     let fruit = Fruit {
//         apples: 10,
//         bananas: 5,
//     };
//     fruit
// }

// fn print_fruit(fruit: Fruit) {
//     println!(
//         "You have {} apples and {} bananas",
//         fruit.apples, fruit.bananas
//     );
// }

// fn main() {
//     let some_fruit = new_fruit();
//     let updated_fruit = increase_fruit(some_fruit);
//     print_fruit(updated_fruit);
// }

// By removing the unnecessary let statements, the code becomes more concise and avoids unnecessary variable assignments.

// Please note that the revised code is functionally equivalent to the original code, but it has been modified to eliminate the let statements for improved readability and simplicity.

#[derive(Debug)]
struct Fruit {
    apples: i32,
    bananas: i32,
}

impl Fruit {
    fn new(apples: i32, bananas: i32) -> Self {
        Fruit { apples, bananas }
    }
    fn increase(self) -> Self {
        Fruit {
            apples: self.apples * 2,
            bananas: self.bananas * 3,
        }
    }
}

fn print_fruit(fruit: Fruit) {
    println!(
        "You have {} apples and {} bananas",
        fruit.apples, fruit.bananas
    );
}

fn main() {
    print_fruit(Fruit::new(10, 5).increase());
}
