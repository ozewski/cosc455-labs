// Chapters 1-5 of the linkedin learning course to learn Rust
// My answers to the challenges are in functions at the top.
// The main method consists of my code notes following along with the videos.

fn min_max_mean() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32 = 0;
    let mut min: i32 = 0;
    let mut mean = 0.0;

    for number in numbers {
        mean += number as f64;
        if number > max {
            max = number;
        }
        if number < min {
            min = number;
        }
    }

    mean /= numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Tests passed!");
}

fn average() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed!");
}

fn temperature_challenge() {
    fn celsius_to_fahrenheit(celsius: f64) -> f64 {
        (1.8 * celsius) + 32.0
    }

    let celsius_temp = 23.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);

    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test passed!");
}

fn main() {
    // CHAPTER 1

  println!("Hello, world!"); // indentation doesn't matter: "free-form" language
    println!("Hello, world!");

    // add comments using double forward slash
    /* 
    java-like 
    multi-line comments 
    */

    // CHAPTER 2

    // declaring variables

    let x = 10;
    println!("x is {}", x);
    // x = 20; // won't work, since variables are immutable by default

    let mut x1 = 10;
    println!("x is {}", x1);

    // integer data types

    let _eight_bit: u8 = 1;
    let _sixteen_bit: u16 = 1;
    let _thirty_two_bit: u32 = 1; // default int type
    let _sixty_four_bit: u64 = 1;
    let _one_hundred_twenty_eight_bit: u128 = 1;

    // signed and out-of-range errors caught by compiler: below examples will throw error
    // let my_var: u8 = -10;
    // let my_var: u8 = 1000;

    // floating-point data types

    let fl = 10.12782345846538745692354;
    println!("x is {}", fl);
    // output is cut off at a certain point because fl64 is the default so it cannot store more precision

    // arithmetic operations

    let a = 10;
    let b = 3;

    println!("c is {}", a + b);
    println!("c is {}", a - b);
    println!("c is {}", a * b);
    println!("c is {}", a / b); // two integers
    println!("c is {}", a % b);

    let a1 = 10.0;
    // println!("c is {}", a1 / b);
    // will not work: cannot divide incompatible types
    // must use casting:
    println!("c is {}", a1 / b as f64);

    // formatting print statements

    println!("{:.3}", 10.0 / 3.0); // pad to an amount of decimal places
    println!("{:8.3}", 10.0 / 3.0); // pad the front with spaces
    println!("{:08.3}", 10.0 / 3.0); // pad the front with zeros
    print!("hello world\n"); // no newline at the end by default
    println!("{1:08.3}, {1}, {0}", 10, 3); // positional arguments

    // bitwise operations

    let binvalue = 0b1111_0101u8;
    println!("{:08b}", binvalue); // format binary
    println!("{:08b}", !binvalue); // bitwise not
    println!("{:08b}", binvalue & 0b1111_0111); // bitwise and
    println!("{:08b}", binvalue | 0b1111_0111); // bitwise or
    println!("{:08b}", binvalue ^ 0b1111_0111); // bitwise xor
    println!("{:08b}", binvalue << 4); // bitwise left-shift
    println!("{:08b}", binvalue >> 4); // bitwise right-shift

    // boolean data type and operations

    // this is all just stuff from discrete math and 290, I don't need to write it out
    // double pipe and double and operators are short-circuiting

    println!("{}", (true && true) || panic!());
    // panic never runs because of short circuit

    // comparison operators

    // same as python/java
    // cannot compare objects of different data types (generally)

    // char data types

    // stores using 4 bytes
    // use single quotes

    let letter = 'a';
    let number = '1';
    let finger = '\u{261D}';
    println!("{}\n{}\n{}", letter, number, finger);

    average();

    // CHAPTER 3

    // arrays

    // fixed length
    let mut letters = ['a', 'b', 'c'];
    letters[0] = 'x';
    println!("first letter is {}", letters[0]);

    let numbers: [i32; 5];
    // println!("fourth number is {}", letters[4]);
    // this will not work

    numbers = [0; 5]; // repeat expression
    println!("length is {}", letters.len());
    // len is of type usize

    // multi-dimensional arrays

    let parking_lot = [[1, 2, 3], [4, 5, 6], [9; 3]];
    let garage: [[[u64; 100]; 20]; 5];

    // tuples

    // can be mixed data types
    // ordered
    // fixed length
    // data types must be known at compile time

    let mut stuff = (10, 3.14, 'x');
    println!("first item is {}", stuff.0); // different syntax from arrays
    stuff.0 += 3;
    println!("first item is {}", stuff.0);

    // destructuring
    let (stufffirst, stuffsecond, stuffthird) = stuff;

    // CHAPTER 4

    // function parameters

    fn say_the_sum(a: u8, b: u8) { // a, b are variables, i8 is type
        println!("sum is {}", a + b);
    }

    say_the_sum(13, 15);
    say_the_sum(14, 15);
    say_the_sum(15, 15);

    let x = 5;
    say_the_sum(12, x);
    // this lets compiler infer x as u8

    // function return values

    fn square(x: i32) -> i32 {
        println!("squaring {x}");
        x * x
        // no semicolon on last line of function makes rust treat it as return value
    }

    let squared = square(8);
    println!("result: {squared}");

    fn square2(x: i32) -> (i32, i32) {
        println!("squaring {x}");
        (x, x * x) // tuple return value
    }

    // unit data type: default data type

    temperature_challenge();

    // CHAPTER 5

    // conditional execution

    let x = 3;

    if x == 3 {
        println!("x is 3!");
    }

    let x = 4;

    if x == 3 {
        println!("x is 3!");
        // will never run
    }

    /*
    if x {
        // will not compile, since x is an int, not a bool
    }
    */

    // multiple conditions

    let x = 3;
    let y = 5;

    if x > y {
        println!("x is greater than y");
    } else if x == y {
        println!("x is equal to y");
    } else {
        println!("x is NOT greater than y");
    }

    // conditional assignment

    let make_x_odd = true;
    let x;

    if make_x_odd {
        x = 1;
    } else {
        // see error upon reference
        // x = 2;
    }

    // println!("x is {x}");
    // will throw possibly uninitialized error since else assignment is commented out

    let x = if make_x_odd {1} else {2};
    // more compact conditional assignment syntax
    
    // let x = if make_x_odd {1} else {2.0};
    // will not work because data types of if and else are different

    // loops

    let mut count = 0;

    loop {
        count += 1;
        println!("count is {count}");

        if count == 10 {
            break
        }
    };

    println!("outside the loop");

    let mut count = 0;

    let result = loop {
        count += 1;
        println!("count is {count}");

        if count == 10 {
            break count * 10;
        }
    }; // semicolon required now that it is a statement

    // while loops

    let mut count = 0;

    while count < 10 {
        count += 1;
        println!("count is {count}");
        // while loop cannot use break to return a value
    }

    // for loops

    let message = ['h', 'e', 'l', 'l', 'o'];

    for item in message {
        println!("item is {item}");
        // creates an iterator under the hood
    }

    for (i, item) in message.iter().enumerate() {
        println!("item is {item}, index {i}");
    }

    // ranges
    for number in 0..5 {
        // prints 0 through 4 inclusive
        println!("number is {number}")
    }

    // nested loops
    let mut matrix = [
        [1, 2, 3],
        [4, 5, 6],
        [7, 8, 9]
    ];

    for row in matrix {
        for num in row {
            print!("{num} ");
            // print numbers in row on the same line
        }
        println!(); // insert new line
    }

    for row in matrix.iter_mut() {
        for num in row.iter_mut() {
            *num += 10;
        }
        println!(); // insert new line
    }

    min_max_mean();

}
