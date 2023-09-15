// Complete the code below to create a linked list using Box smart pointers. Each Node struct has a data field of type i32 and a next field of type Option<Box<Node>>. Your task is to assign the next field of each node appropriately to create a linked list. The final linked list should start with node1, followed by node2, and finally node3. Print the linked list at the end.

// The expected output should be:

// Linked list: Node { data: 1, next: Some(Node { data: 2, next: Some(Node { data: 3, next: None }) }) }

// #[derive(Debug)]
// struct Node {
//     data: i32,
//     next: Option<Box<Node>>,
// }

// fn main() {
//     let mut node1 = Node {
//         data: 1,
//         next: None,
//     };

//     let mut node2 = Node {
//         data: 2,
//         next: None,
//     };

//     let mut node3 = Node {
//         data: 3,
//         next: None,
//     };

// Insert your code here which will connect the nodes using Box pointers. Two statements are enough here

//     println!("Linked list: {:?}", node1);
// }

use std::io::stdin;

type Pointer<T> = Option<Box<T>>;

#[derive(Debug)]
struct Node {
    data: i32,
    next: Pointer<Node>,
}

impl Node {
    fn new(data: i32, next: Pointer<Node>) -> Self {
        Node { data, next }
    }
}

#[derive(Debug)]
struct LinkedList {
    head: Pointer<Node>,
}

impl LinkedList {
    fn new_list(head: Pointer<Node>) -> Self {
        LinkedList { head }
    }

    fn push(&mut self, data: i32) {
        let current_head = self.head.take();
        let new_head = Node::new(data, current_head);
        self.head = Some(Box::new(new_head));
    }

    fn print(&self) {
        if self.head.is_none() {
            println!("La lista esta vacía")
        } else {
            let mut node = &self.head;

            while node.is_some() {
                let node_to_print = node.as_ref().unwrap();
                print!("{}, ", node_to_print.data);
                node = &node_to_print.next;
            }
        }
    }
}

fn main() {
    let number_of_nodes = ask_user_for_a_number("Ingrese un número", 3);

    let mut nodes = LinkedList::new_list(None);

    for i in 0..number_of_nodes {
        nodes.push(number_of_nodes - i)
    }

    println!("Linked list: \n {:#?}", nodes);
    nodes.print()
}

fn ask_user_for_a_number(label: &str, default_value: i32) -> i32 {
    let mut user_input = String::new();

    println!("{label}");
    stdin()
        .read_line(&mut user_input)
        .expect("Error al leer el input del usuario");

    let input_as_number = user_input.trim().parse().unwrap_or(default_value);

    input_as_number
}
