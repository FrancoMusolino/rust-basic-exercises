//3) You are developing a text processing utility in Rust. Your task is to implement a function called find_longest_word() that takes a string slice as input and returns a reference to the longest word in that string slice. You will need to handle cases where there are multiple words with the same length, in which case the function should return the first occurrence.

// Write the find_longest_word() function, ensuring that the lifetime parameter is properly handled to ensure the validity of the returned reference.

use std::io::stdin;

fn ask_user_for_string(label: &str) -> String {
    let mut input = String::new();

    println!("{label}");
    stdin()
        .read_line(&mut input)
        .expect("Error al leer lo ingresado por el usuario");

    input
}

fn find_longest_word(text: &String) -> String {
    let mut longest_word = String::new();
    let splitted_text = text.split_whitespace();

    for word in splitted_text {
        if longest_word.is_empty() || longest_word.len() < word.len() {
            longest_word = word.to_string()
        }
    }

    longest_word
}

fn main() {
    let text_to_process = ask_user_for_string("Ingrese un texto (frase, oración, etc)");

    let longest_word = find_longest_word(&text_to_process);

    println!("\nTexto ingresado: {}", text_to_process);
    println!(
        "La primera palabra con {} caracteres, el máximo valor encontrado en el texto, es: {}",
        longest_word.len(),
        longest_word
    )
}
