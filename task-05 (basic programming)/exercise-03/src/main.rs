// 3) This question involves writing code to analyze the production of an assembly line in a car factory. The assembly line has different speeds, ranging from 0 (off) to 10 (maximum). At the lowest speed of 1, the assembly line produces a total of 221 cars per hour. The production rate increases linearly with the speed, meaning that a speed of 4 produces 4 * 221 = 884 cars per hour.

// However, higher speeds increase the likelihood of producing faulty cars that need to be discarded. The success rate depends on the speed, as shown in the table below:

// · Speeds 1 to 4: 100% success rate.

// · Speeds 5 to 8: 90% success rate.

// · Speeds 9 and 10: 77% success rate.

// You need to write two functions:

// 1. The first function, total_production(), calculates the total number of cars successfully produced without faults within a specified time given in hours. The function takes the number of hours and speed as input and returns the number of cars successfully produced.

// 2. The second function, cars_produced_per_minute(), calculates the number of cars successfully produced per minute. The function takes the number of hours and speed as input and returns the number of cars produced per minute.

// Write the code for both functions based on the provided specifications.

use std::io::stdin;

const MIN_CARS_PRODUCED_PER_HOUR: f32 = 221.0;

fn total_production(production_hours: u32, speed: u8) -> f32 {
    let success_rate: f32;

    if speed <= 4 {
        success_rate = 1.0;
    } else if speed >= 5 && speed <= 8 {
        success_rate = 0.9;
    } else {
        success_rate = 0.77;
    }

    (MIN_CARS_PRODUCED_PER_HOUR * (production_hours as f32)) * success_rate * (speed as f32)
}

fn production_per_minute(cars_produced: f32, hours: u32) -> f32 {
    cars_produced / (60.0 * (hours as f32))
}

fn ask_user_for_number(label: &str) -> u32 {
    let mut input = String::new();

    println!("{label}");

    stdin()
        .read_line(&mut input)
        .expect("Error al leer el input del usuario");

    input
        .trim()
        .parse::<u32>()
        .expect("El valor ingresado no es un número positivo")
}
fn main() {
    let hours = ask_user_for_number("Ingrese la cantidad de horas de producción");

    let speed_num = ask_user_for_number("Ingrese la velocidad de producción");
    let speed = match speed_num {
        1..=10 => speed_num,
        _ => panic!("La velocidad de producción se mide en una escala de 1 a 10 inclusive"),
    };

    let cars_produced_without_faults = total_production(hours, speed as u8);
    let car_produced_per_minute = production_per_minute(cars_produced_without_faults, hours);

    println!("El total de autos producidos, sin fallas, en {} horas a una velocidad de producción {} es de: {}", hours, speed, cars_produced_without_faults);
    println!(
        "El total de autos producidos, sin fallas, por minuto es de {}",
        car_produced_per_minute
    )
}
