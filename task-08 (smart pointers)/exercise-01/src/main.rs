use std::io::stdin;

fn ask_user_for_string(label: &str) -> String {
    let mut input = String::new();

    println!("{label}");
    stdin()
        .read_line(&mut input)
        .expect("Error al leer el mensaje del usuario");

    input.trim().to_string()
}

type Pointer<T> = Option<Box<T>>;

#[derive(Debug)]
struct Node<'a> {
    element: &'a str,
    next: Pointer<Node<'a>>,
}

impl<'a> Node<'a> {
    fn new(element: &'a str, next: Pointer<Node<'a>>) -> Self {
        Node { element, next }
    }
}

#[derive(Debug)]
struct LinkedList<'a> {
    head: Pointer<Node<'a>>,
}

impl<'a> LinkedList<'a> {
    fn empty_list() -> Self {
        LinkedList { head: None }
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn push(&mut self, element: &'a str) {
        let prev_head = self.head.take();
        let new_head = Node::new(element, prev_head);
        self.head = Some(Box::new(new_head));
    }

    fn print(&self) {
        if self.is_empty() {
            println!("La lista esta vacía")
        } else {
            let mut node = &self.head;

            while node.is_some() {
                let unwrapped_node = node.as_ref().unwrap();
                print!("{} ", unwrapped_node.element);
                node = &unwrapped_node.next;
            }

            println!()
        }
    }
}

fn split_words_into_linked_list(text: &str) -> LinkedList {
    let mut linked_list = LinkedList::empty_list();
    let text_as_bytes = text.as_bytes();

    let mut from = 0;

    for (index, word) in text_as_bytes.iter().enumerate() {
        if *word == b' ' {
            let slice_to_insert = &text[from..index];

            if !slice_to_insert.is_empty() {
                linked_list.push(slice_to_insert);
            }

            from = index + 1;
        }

        if index + 1 == text_as_bytes.len() {
            linked_list.push(&text[from..=index]);
        }
    }

    linked_list
}

fn main() {
    let user_input = ask_user_for_string("Ingrese un texto / frase");

    let result = split_words_into_linked_list(&user_input);
    result.print();
    println!("{:#?}", result)
}
