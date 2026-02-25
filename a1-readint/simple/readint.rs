use std::io::{self, Write};  // import 'Write' and 'io' (= self)
use std::process::exit;


// read a 32-bit integer from stdin
fn read_int() -> i32 {
    print!("Please enter a 32-bit integer: "); 
    io::stdout().flush().unwrap(); // flush to ensure prompt is shown

    let mut input = String::new();

    // read a line from stdin => String
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // trim newline (remove white spaces) and parse to i32
    let value: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: not a valid 32-bit integer!");
            exit(1);
        }
    };
    value
}

fn main() {
    let value = read_int();
    println!("value = {}", value);
}
