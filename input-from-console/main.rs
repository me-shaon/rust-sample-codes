use std::io;
use std::io::prelude::*;

/* Simple single line String input */
fn single_string_input() {
    let mut in_str = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut in_str).unwrap();
    println!("{}", in_str);
}

/* Integer number input */
fn single_integer_number_input() {
    let mut in_str = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut in_str).unwrap();

    /* Method: 1
        Following code will proceed if the given value is a valid integer
        Otherwise it will give a panic and the program will stop executing
    */
    // let int_val: i64 = in_str.trim().parse().expect("Not a valid integer");

    /* Following is just a rewrite of the above line */
    // let int_val = in_str.trim().parse:<i64>().expect("Not a valid integer");

    /* Method: 2
        Follwing code is using 'pattern matching' to check if it is a valid integer
        If not, it can assign a default fallback value
    */
    let int_val = match in_str.trim().parse::<i64>() {
        Ok(n) => n,
        Err(_) => 0
    };


    /* Method: 3
        Following code is almost similar like the Method-2
        It is just using 'if let' instead of 'pattern matching'
    */
    // let mut int_val = 0;
    // if let Ok(n) = in_str.trim().parse::<i64>() {
    //     int_val = n;
    // }

    println!("{}", int_val);
}

/* Multiple String Line input */
fn infinite_string_input() {
    let stdin = io::stdin();

    /* Take line input until user stop it by pressing Ctrl+D or Ctrl+C */
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}

/* Main Function */
fn main() {

    println!("Enter a single string: ");
    single_string_input();

    println!("Enter an integer number: ");
    single_integer_number_input();

    println!("Enter multiple string (Ctrl+D to Quit): ");
    infinite_string_input();
}
