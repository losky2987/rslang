use std::fs::File;
use std::io::Read;
use std::io;

fn main() {
   let ret = read_file();
   match ret {
       Ok(s) => println!("File content: {}", s),
       Err(e) => println!("Error reading file: {}", e),
   }

}

fn read_file() -> Result<String, io::Error> {
   let mut f = File::open("hello.txt")?;
   let mut s = String::new();
   f.read_to_string(&mut s)?;
   Ok(s)
}
