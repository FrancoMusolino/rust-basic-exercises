// Consider the code below. Write the expanded version of the code that can be viewed using the cargo expand utility.

macro_rules! recursively_sum {
    ($initial_value: expr, ($($addends: expr),+)) => {{
            let mut sum = $initial_value;
            $(sum += $addends;)+
            sum
    }};
}

macro_rules! recursively_substract {
    ($initial_value: expr, ($($subtracting: expr),+)) => {{
        let mut substraction = $initial_value;
        $(substraction -= $subtracting;)+
        substraction
    }};
}

fn main() {
    let acc = recursively_sum!(40, (1, 2, 3, 4, 5));
    println!("{acc}");

    let res = recursively_substract!(100, (12, 43, 6));
    println!("{res}");
}
