fn main() {
    let mut str:String = String::from("This is a string, ");
    string_borrow(&mut str); // This is a string, EOF
    string_borrow(&mut str); // This is a string, EOFEOF
    println!("{}", str);
}

fn string_borrow(str: &mut String) {
    str.push_str("EOF");
}