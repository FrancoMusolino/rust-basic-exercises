// 2) In this question, you will solve the same problem as discussed in Question 1, but using arrays.

// First, declare two arrays called p1 and p2. These arrays will have a length of 2 and a type of f64. Each array represents a point, with the x-axis value in the first index (position) and the y-axis value in the second index (position).

// Next, write a program that calculates and displays the absolute difference between the x-axis values and the absolute difference between the y-axis values.

// Note: Use the abs() function to determine the absolute value of the difference. In this case, you don't need to explicitly specify the types as f64 for all the values because the compiler can infer the types correctly. The compiler is smart enough to detect the types being passed to the abs() function, so you won't encounter any issue

fn main() {
    let p1: [f64; 2] = [-32.5, 2.21];
    let p2: [f64; 2] = [14.32, -8.0];

    let x_axis_difference = (p1[0] - p2[0]).abs();
    let y_axis_difference = (p1[1] - p2[1]).abs();

    println!(
        "The difference in X is {} and in Y is {}",
        x_axis_difference, y_axis_difference
    )
}
