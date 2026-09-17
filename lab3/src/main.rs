// Chapters 1-5 of the linkedin learning course to learn Rust

// add comments using double forward slash
/* 
java-like 
multi-line comments 
*/

fn main() {
    // CHAPTER 1

  println!("Hello, world!"); // indentation doesn't matter: "free-form" language
    println!("Hello, world!");

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