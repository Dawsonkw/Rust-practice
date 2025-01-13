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


//  fn main() {
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {}", y);
//  }

// Functions with return values
// fn five() -> i32 {
//     5
// }

// fn main() {
//     let x = five();

//     println!("The value of x is: {}", x);
// }

// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {}", x);
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// Control flow

// fn main() {
//     let number = 3;

//     if number <5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

    // fn main() {
    //     let number: i64 = 566456;

    //     if number % 4 == 0 {
    //         println!("number is divisible by 4");
    //     } else if number % 3 == 0 {
    //         println!("number is divisible by 3");
    //     } else if number % 2 == 0 {
    //         println!("number is divisible by 2");
    //     } else {
    //         println!("number is not divisible by 4, 3, or 2");
    //     }
    // }

    // fn main() {
    //     let condition = true;

    //     let number = if condition { 5 } else { 6 };

    //     println!("The value of number is: {}", number);
    // }

    // fn main() {
    //     loop {
    //         println!("again!");
    //     }
    // }

    // fn main() {
    //     let mut counter = 0;

    //     let result = loop {
    //         counter += 1;

    //         if counter == 42069 {
    //             break counter * 2;
    //         }
    //     };

    //     println!("The result is {}", result);
    // }

    // fn main() {
    //     let mut count = 0;

    //     'counting_up: loop{
    //         println!("count is: {count}");
    //         let mut remaining = 10;

    //         loop {
    //             println!("remaining is: {remaining}");
    //             if remaining == 9 {
    //                 break;
    //             }
    //             if count == 2 {
    //                 break 'counting_up;
    //             }
    //             remaining -= 1;
    //         }

    //         count += 1;
    //     }

    //     println!("End count is: {count}");
    // }

    // fn main() {
    //     let mut number = 3;

    //     while number != 0 {
    //         println!("{}", number);

    //         number -= 1;
    //     }

    //     println!("LIFTOFF!!");
    // }

    // fn main() {
    //     let a = [10, 20, 30, 40, 50];
    //     let mut index = 0 ;

    //     while index < 5 {
    //         println!("the value is: {}", a[index]);
    //         index += 1;
    //     }
    // }

    // fn main() {
    //     let a = [10, 20, 30, 40, 50];

    //     for element in a {
    //         println!("the value is: {}", element);
    //     }
    // }

    // fn main() {
    //     for number in (1..4).rev() {
    //         println!("{}!", number);
    //     }
    //     println!("LIFTOFF!!");
    // }


// Chapter projects 



    //Convert temperatures between Fahrenheit and Celsius.
    fn main() {
        // Get the temperature input from the user
        println!("Enter a temperature in Fahrenheit:");
        let mut fahrenheit = String::new();
        std::io::stdin().read_line(&mut fahrenheit).expect("Failed to read line");
        let fahrenheit: f32 = fahrenheit.trim().parse().expect("Please enter a valid number");

        // Convert the temperature to Celsius and Kelvin
        let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
        let kelvin = celsius + 273.15;

        // Print the result
        println!("{} degrees Fahrenheit is equal to {} degrees Celsius, and {} degrees Kelvin", fahrenheit, celsius, kelvin);

    }

    // Generate the nth Fibonacci number.

    // Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.