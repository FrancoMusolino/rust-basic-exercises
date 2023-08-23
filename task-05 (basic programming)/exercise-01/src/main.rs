// 1) Write a program to find the difference between the square of the sum and the sum of the squares of the first N numbers. N will be a user-defined input that your program will take.

// For example, if the user enters the number 5, you should compute the square of the sum as (1 + 2 + 3 + 4 + 5)^2 = 225.

// Next, compute the sum of the squares as (1^2 + 2^2 + 3^2 + 4^2 + 5^2) = (1 + 4 + 9 + 16 + 25) = 55.

// Finally, calculate the difference as 225 - 55 = 170.

use std::io::stdin;

fn square_of_sum(num: i32) -> i32 {
    let mut sum_total = 0;

    for i in 1..=num {
        sum_total += i
    }

    sum_total.pow(2)
}

fn sum_of_squares(num: i32) -> i32 {
    let mut sum_total = 0;

    for i in 1..=num {
        sum_total += i.pow(2)
    }

    sum_total
}

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

    let square_of_sum = square_of_sum(input_as_number);
    let sum_of_squares = sum_of_squares(input_as_number);

    let final_result = square_of_sum - sum_of_squares;

    println!("El resultado final es {}", final_result)
}
