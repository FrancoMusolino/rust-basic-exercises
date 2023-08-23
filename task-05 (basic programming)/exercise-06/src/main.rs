// 6) Write a function that implements the logic, 'You can see the movie if you are 17 or older, or if you are 13 or older and have a parent's permission.'

// Use the following skeleton of the function. Remove the 'return false' statement once you have written the code inside the function.

// fn can_see_movie(age: i32, permission: bool) -> bool {

//     // Write your code here

//     // Remove 'return false' once you have written the code

//     return false;

// }

// Please fill in the code inside the function to implement the logic described above. Once you have completed the implementation, remove the 'return false' statement.

fn can_see_movie(age: i32, permission: bool) -> bool {
    return age >= 17 || (age >= 13 && permission);
}
fn main() {
    let can_see_movie = can_see_movie(17, false);
    println!("¿Puede ver la película? {}", can_see_movie)
}
