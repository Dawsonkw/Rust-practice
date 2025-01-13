// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {}", x);
//     }

//     println!("the value of x is: {}", x);
// }


// fn main() {
//     //addition
//     let sum = 5 + 10;

//     //subtraction
//     let difference = 95.5 - 4.3;

//     //multiplication
//     let product = 4 * 30;

//     //division
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // Results in -1

//     //remainder
//     let remainder = 43 % 5;

//     println!("sum: {}, difference: {}, product: {}, quotient: {}, truncated: {}, remainder: {}", sum, difference, product, quotient, truncated, remainder);


// }

//  fn main() {
//     let t = true;
//     let f: bool = false; // with explicit type annotation

//     println!("t: {}, f: {}", t, f);
// }

// fn main() {
//     let c = 'z';
//     let z = 'Z';
//     let heart_eyed_cat = '😻';

//     println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);
// }

// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {}, z is: {}, x is: {}", y, z, x);
// }

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;
//     let six_point_four = x.1;
//     let one = x.2;

//     println!("The value of y is: {}, z is: {}, x is: {}", six_point_four, one, five_hundred);
// }


//Displaying rusts memory safe principles in action
// use std::io;
// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {} is: {}", index, element);
// }


//Functions
    // fn main() {
    //     println!("Hello, world!");

    //     another_function();
    // }
    
    // fn another_function() {
    //     println!("Another function.");
    // }

    // fn main() {
    //     print_labeled_measurement(5, 'h');
    // }

    // fn print_labeled_measurement(value: i32, unit_label: char) {
    //     println!("the measurement is: {value}{unit_label}");
    // }