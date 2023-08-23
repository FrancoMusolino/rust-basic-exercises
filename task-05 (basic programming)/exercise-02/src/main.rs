// 2) Write a program to find the sum of natural numbers below a given number N, where N is provided by the user. The sum should only include numbers that are multiples of either 3 or 5.

// For example, if the user enters N = 20, the multiples of 3 are 3, 6, 9, 12, 15, 18, and the multiples of 5 are 5, 10, and 15. Please note that the value of 15 will be considered only once since it is a multiple of both 3 and 5.

// The sum will be calculated as follows: 3 + 5 + 6 + 9 + 10 + 12 + 15 + 18.

// Write a program that takes the user input N, performs the necessary calculations, and outputs the sum.

use std::io::stdin;

fn main() {
    let mut input = String::new();

    println!("Ingrese un número:");
    stdin()
        .read_line(&mut input)
        .expect("Error al leer el input del usuario");

    let input_as_number: i32 = input
        .trim()
        .parse::<i32>()
        .expect("El valor ingresado no es un número");

    let mut acc = 0;

    for i in 1..input_as_number {
        if i % 3 == 0 || i % 5 == 0 {
            acc += i
        }
    }

    println!(
        "La suma de los números naturales divisibles por 3 o 5 menores a {} es: {}",
        input_as_number, acc
    )
}
