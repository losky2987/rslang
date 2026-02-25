mod inner;

use inner::more::add_three as add3;

fn outer() {
   let mut x = 160;
   inner::add_two(&mut x);
   add3(&mut x);
}

fn main() {
   outer();
}
