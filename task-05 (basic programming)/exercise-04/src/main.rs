// 4) A palindrome is a word, verse, or sentence that reads the same backward or forward, such as 'Able was I ere I saw Elba,' or a number like 1881.

// Write a function named is_palindrome() that checks whether a given string is a palindrome or not. The function should take a string as input and return a boolean value indicating whether the string is a palindrome or not.

use std::io::stdin;

fn ask_user_for_string(label: &str) -> String {
    let mut input = String::new();

    println!("{label}");

    stdin()
        .read_line(&mut input)
        .expect("Error al leer el input del usuario");

    input.trim().to_string()
}

fn is_palindrome(text: &String) -> bool {
    let text_lowercase = text.to_lowercase();
    let text_bytes = text_lowercase.as_bytes();

    let mut first = 0;
    let mut last = text_bytes.len() - 1;

    while first < last {
        if text_bytes[first] != text_bytes[last] {
            return false;
        }

        first += 1;
        last -= 1;
    }

    true
}

fn main() {
    let user_input = ask_user_for_string("Ingrese una palabra o frase");
    let result = is_palindrome(&user_input);

    println!(
        "¿La palabra/frase {} es palíndromo?: {}",
        user_input, result
    )
}
