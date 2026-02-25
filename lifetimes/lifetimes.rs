fn largest<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y { x } else { y }
}

fn foo(x: &i32) -> &i32 {
    return x;
}

fn main() {
   let a = 1;
   let b = 2;   
   let res = largest(&a, &b);
   println!("Largest is {}", res);
   foo(&a);
}



