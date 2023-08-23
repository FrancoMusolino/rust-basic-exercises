// 1) In this exercise, you will create two tuples, p1 and p2, representing points on a Cartesian plane. Each tuple will contain two values, one for the x-axis and another for the y-axis. Your task is to write a program that calculates and displays the absolute difference between the x-axis values and the absolute difference between the y-axis values.

// Note: To calculate the absolute value, use the abs() function. The compiler may show an error message stating "ambiguous numeric type" when using this function. To resolve this, ensure that you specify the value as f64 by writing "as f64" in front of it. For example, (-3.5 as f64).abs() will result in a value of 3.5.

struct CartesianPoints(f64, f64);

impl CartesianPoints {
    fn new(x: f64, y: f64) -> Self {
        CartesianPoints(x, y)
    }
}

fn main() {
    let p1 = CartesianPoints::new(-32.5, 2.21);
    let p2 = CartesianPoints::new(14.32, -8.0);

    let x_axis_difference = (p1.0 - p2.0).abs();
    let y_axis_difference = (p1.1 - p2.1).abs();

    println!(
        "The difference in X is {} and in Y is {}",
        x_axis_difference, y_axis_difference
    )
}
