use std::io::{self, Write};
use std::error::Error;
use std::process::exit;


// read a 32-bit integer from stdin
// Returns a Result, Box<dyn Error> is an owned trait object encapsulating any error type
fn read_int() -> Result<i32, Box<dyn Error>> {
    print!("Please enter a 32-bit integer: "); 
    io::stdout().flush()?; // flush to ensure prompt is shown

    let mut input = String::new();

    // read a line from stdin => String
    io::stdin().read_line(&mut input)?;

    // trim newline (remove white spaces) and parse to i32
    let res_ok = input.trim().parse::<i32>()?;
    Ok( res_ok )
}

fn main() {
    let value = read_int();

    match value {
        Ok(num) => println!("num = {}", num),
        Err(e) => {
            eprint!("Error: not a valid 32-bit integer!");
            eprintln!(" {:?}", e);
            exit(1);
        }
    }
}
