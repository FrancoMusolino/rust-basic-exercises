// 1) Write a program that calculates the set intersection and set union of two vectors containing some numbers. The program should have two separate functions: one for calculating the intersection and another for calculating the union.

// The program will define two input vectors, vec_1 and vec_2, which contain unsigned 32-bit integers. For example:

// let mut vec_1: Vec<u32> = vec![5, 4, 3, 6, 9];

// let mut vec_2: Vec<u32> = vec![5, 8, 6, 4, 10, 15, 20, 21];

// These vectors will be passed as arguments to the intersection and union functions to compute the respective results.

fn compute_intersection(first_vec: &Vec<u32>, second_vec: &Vec<u32>) -> Vec<u32> {
    let mut common = Vec::new();

    for num in first_vec {
        let exists_in_both = second_vec.iter().any(|x| x == num);

        if exists_in_both {
            let already_pushed_into_common = common.iter().any(|x| x == num);

            if !already_pushed_into_common {
                common.push(*num);
            }
        }
    }

    common
}

fn compute_union<'a>(
    first_vec: &'a mut Vec<u32>,
    second_vec: &'a mut Vec<u32>,
    common: &'a Vec<u32>,
) -> Vec<&'a u32> {
    for i in common {
        let mut common_index_in_first = first_vec.iter().position(|x| x == i);
        while common_index_in_first != Option::None {
            first_vec.remove(common_index_in_first.unwrap());
            common_index_in_first = first_vec.iter().position(|x| x == i);
        }

        let mut common_index_in_second = second_vec.iter().position(|x| x == i);
        while common_index_in_second != Option::None {
            second_vec.remove(common_index_in_second.unwrap());
            common_index_in_second = second_vec.iter().position(|x| x == i);
        }
    }

    let union_vec = first_vec
        .iter()
        .chain(second_vec.iter())
        .chain(common.iter())
        .collect::<Vec<_>>();

    union_vec
}

fn main() {
    let mut vec_1: Vec<u32> = vec![5, 4, 3, 6, 9, 5, 5, 5, 5, 32, 10, 21];
    let mut vec_2: Vec<u32> = vec![5, 8, 6, 4, 5, 5, 5, 10, 10, 10, 10, 10, 11, 15, 20, 21];

    let mut common_values = compute_intersection(&vec_1, &vec_2);
    common_values.sort();

    println!("The common values are {:?}", common_values);

    let mut union_values = compute_union(&mut vec_1, &mut vec_2, &common_values);
    union_values.sort();

    println!("The union of values is {:?}", union_values)
}
