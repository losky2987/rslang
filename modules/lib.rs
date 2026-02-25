mod inner;

fn outer() {
   let mut x = 160;
   inner::add_two(&mut x);
}

fn main() {
   outer();
}
