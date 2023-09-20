// We want to create a macro called make_struct which will create a new struct containing some fields. The input to the macro is the name of the struct and the name of the fields along with their types. The skeleton of the macro along with its left sides of the rules are given.

// You are required to write the code for the expansion or the right side of the rule.

// macro_rules! make_struct {
//     ($name:ident {$($field:ident: $ty:ty),* }) => {s
//        // Your code here
//         }
//     };
// }

macro_rules! make_struct {
    ($name: ident {$($field: ident: $ty: ty);+}) => {
        struct $name {
            $($field: $ty), +
        }
    };
}

make_struct!(Person {
    id: i32;
    name: String;
    age: u8
});

fn main() {
    let user = Person {
        id: 8346938,
        name: "Juan".to_string(),
        age: 2,
    };

    println!("User id: {}", user.id);
    println!("User name: {}", user.name);
    println!("User age: {}", user.age)
}
