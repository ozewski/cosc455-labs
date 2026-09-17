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

}