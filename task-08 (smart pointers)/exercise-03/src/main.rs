// You are developing a messaging application where users can send messages to each other. As part of the application, you need to create a Message struct that represents a single message. The struct should have fields for the sender's name and the content of the message.

// Currently, the messaging functionality is not implemented, but it will be added in future updates of the application. When a message is sent out to multiple users, all of them should have ownership of the message. This is required to ensure that the user are able to read the message and consume it successfully.

// To achieve this shared ownership, you need to use the Rc (Reference Counted) smart pointer. Rc allows multiple users to have shared ownership of the messages. It keeps track of the number of references to the message and deallocates the message when the last reference goes out of scope.

// Your task is to implement the following functions for the Message struct:

// new(): Takes the sender's name and message content as parameters and returns a new Message instance wrapped in an Rc.

// sender_name: Returns a reference to the sender's name.

// message_content: Returns a reference to the message content.

// Use the code below for complete

// use std::rc::Rc;

// struct Message {
//     sender_name: String,
//     message_content: String,
// }

// impl Message {
//     fn new(sender_name: String, message_content: String) -> Rc<Self> {
//         // Your code here
//     }

//     fn sender_name(&self) -> &str {
//        // Your code here
//     }

//     fn message_content(&self) -> &str {
//         // Your code here
//     }
// }

// fn main() {
//     let message = Message::new("Alice".to_string(), "Hello, Bob!".to_string());
//     println!("Sender: {}", message.sender_name());
//     println!("Message: {}", message.message_content());
// }

use autoincrement::prelude::*;
use dialoguer::Select;
use std::{cell::RefCell, io::stdin, rc::Rc};

#[derive(Incremental, PartialEq, Eq, Debug)]
struct Id(u32);

#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

impl User {
    fn new(id: u32, name: String) -> Self {
        Self { id, name }
    }
}

type Pointer<T> = Option<Rc<RefCell<T>>>;

#[derive(Debug)]
struct Message {
    sender_name: String,
    content: String,
    prev: Pointer<Message>,
    next: Pointer<Message>,
}

impl Message {
    fn new(sender_name: String, content: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            sender_name,
            content,
            prev: None,
            next: None,
        }))
    }
}

#[derive(Debug)]
struct Chat {
    head: Pointer<Message>,
    tail: Pointer<Message>,
}

impl Chat {
    fn new_empty_chat() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none() || self.tail.is_none()
    }

    fn push_back(&mut self, sender_name: String, content: String) {
        let new_tail = Message::new(sender_name, content);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    fn get_last_message(&self) -> Option<&RefCell<Message>> {
        if self.is_empty() {
            println!("El chat se encuentra vacío");
            None
        } else {
            Some(self.tail.as_ref().unwrap())
        }
    }

    fn print(&self) {
        if self.is_empty() {
            println!("El chat se encuentra vacío");
        } else {
            let mut message = self.head.clone();
            while message.is_some() {
                println!(
                    "{}: {}",
                    message.as_ref().unwrap().borrow().sender_name,
                    message.as_ref().unwrap().borrow().content
                );

                message = message.unwrap().borrow().next.clone();
            }
        }
    }
}

fn main() {
    let mut generator = Id(1).init_from();

    let users: Vec<User> = vec![
        User::new(generator.pull().0, "Juan".to_string()),
        User::new(generator.pull().0, "Pepe".to_string()),
        User::new(generator.pull().0, "Ivan".to_string()),
        User::new(generator.pull().0, "Camilito".to_string()),
    ];

    let user_names = users
        .iter()
        .map(|user| &user.name)
        .collect::<Vec<&String>>();

    let mut chat = Chat::new_empty_chat();

    let mut send_new_message = true;

    while send_new_message {
        let message_content = ask_for_message_content();
        let sender_name = select_sender_user(&user_names);

        chat.push_back(sender_name.clone(), message_content);
        let message_pushed_back = chat.get_last_message().unwrap().borrow();

        println!(
            "Nuevo mensaje enviado por {}: {}",
            message_pushed_back.sender_name, message_pushed_back.content
        );

        send_new_message = respond_to_message();
    }

    println!("Chat:");
    chat.print();
}

fn ask_for_message_content() -> String {
    let mut message = String::new();

    println!("Ingrese el contenido del mensaje");
    stdin()
        .read_line(&mut message)
        .expect("Error al leer el input del usuario");

    message.trim().to_string()
}

fn select_sender_user<'a>(user_names: &Vec<&'a String>) -> &'a String {
    let selection = Select::new()
        .with_prompt("¿Qué usuario ha enviado el mensaje?")
        .items(user_names)
        .interact_opt()
        .expect("Error al seleccionar el sender");

    if selection.is_none() {
        panic!("Debe seleccionar el usuario que envío el mensaje")
    }

    &user_names[selection.unwrap()]
}

fn respond_to_message() -> bool {
    let selection = Select::new()
        .with_prompt("¿Deseas responder al mensaje?")
        .item("Si")
        .item("No")
        .interact_opt()
        .expect("Error al seleccionar el sender");

    if selection.is_none() {
        return false;
    }

    if selection.unwrap() == 0 {
        true
    } else {
        false
    }
}
