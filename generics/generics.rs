struct Point<T> {
    x: T,
    y: T,
}

fn main() {
   let p = Point::<i32> { x: 5, y: 10 };
   let q = Point {x: 1.2, y: 3.4 };

   println!("Point p: ({}, {})", p.x, p.y);
   println!("Point q: ({}, {})", q.x, q.y);
}