// 4) In this exercise, you will be working on creating a student management system using Rust. The system should allow you to store and retrieve student information based on their unique ID. First, you will create the necessary data structures.

// Create a Student structure representing a student with the following fields:

// id: An integer representing the unique ID of the student.

// name: A string representing the name of the student.

// grade: A string representing the grade of the student.

// Next, create a StudentManager structure representing the student management containing a field of student: This will essentially be a hashmap where the key part will be an integer representing unique ID of student and the value part will be the complete details of the students contained in the student structure.

// The StudentManager should implement the following methods:

// 1. new() -> Self: A constructor that initializes an empty student manager.

// 2. add_student(&mut self, student: Student) -> Result<(), String>: Adds a student to the manager. If the student's ID already exists, return an error message. Otherwise, add the student to the manager and return Ok.

// 3. get_student(&self, id: i32) -> Option<&Student>: Retrieves a student from the manager based on their ID. If the student is found, return Some(student). Otherwise, return None.

// Your task is to implement the Student structure, StudentManager structure, and the mentioned methods. Additionally, provide a sample usage of the student management system by adding a few students and retrieving their information using the get_student() method.

use std::{collections::HashMap, io::stdin};

struct Student {
    id: i32,
    name: String,
    grade: String,
}

impl Student {
    fn new(id: i32, name: String, grade: String) -> Self {
        Student { id, name, grade }
    }
    fn print(&self) {
        println!(
            "El estudiante {}, de nombre {}, actualmente se encuentra cursando {}",
            self.id, self.name, self.grade
        )
    }
}

struct StudentsManager {
    students: HashMap<i32, Student>,
}

impl StudentsManager {
    fn new() -> Self {
        StudentsManager {
            students: HashMap::new(),
        }
    }
    fn add_student(&mut self, student: Student) -> Result<(), String> {
        if self.students.contains_key(&student.id) {
            return Err(format!("El ID {} ya se encuentra en uso", student.id));
        }

        self.students.insert(student.id, student);
        Ok(())
    }
    fn get_student(&self, id: i32) -> Option<&Student> {
        self.students.get(&id)
    }
}

fn ask_user_for_number(label: &str) -> i32 {
    let mut input = String::new();

    println!("{label}");
    stdin()
        .read_line(&mut input)
        .expect("Error al leer el input del usuario");

    input.trim().parse().expect("Invalid input, not a number")
}

fn main() {
    let mut students_manager = StudentsManager::new();

    let students: [Student; 5] = [
        Student::new(1, String::from("Juan"), String::from("Primer grado")),
        Student::new(2, String::from("Juancito"), String::from("Segundo grado")),
        Student::new(3, String::from("Pepe"), String::from("Tercero grado")),
        Student::new(4, String::from("Pepito"), String::from("Cuarto grado")),
        Student::new(5, String::from("Lolcito"), String::from("Quinto grado")),
    ];

    for student in students {
        let add_student_result = students_manager.add_student(student);

        match add_student_result {
            Err(msg) => panic!("{msg}"),
            Ok(_) => (),
        }
    }

    let student_id = ask_user_for_number("Ingrese el ID del estudiante");

    let student = match students_manager.get_student(student_id) {
        Some(student) => student,
        None => panic!("No se ha encontrado al estudiante con id {}", student_id),
    };

    student.print();
}
