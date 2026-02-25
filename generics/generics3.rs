struct Point<T> {
    x: T,
    y: T,
}

impl<T: std::ops::AddAssign> Point<T> {

    // Associated function (constructor)
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
 
    // Move the point (mutates it), requires T to implement AddAssign
    fn move_by(&mut self, dx: T, dy: T) {
        self.x += dx;
        self.y += dy;
    }
}

fn main() {
   let mut p = Point::<i32> { x: 5, y: 10 };
   let q = Point::new(1.2, 3.4);

   println!("Point p: ({}, {})", p.x, p.y);
   println!("Point q: ({}, {})", q.x, q.y);
   r.move_by(1,1);
   println!("Point p after move: ({}, {})", p.x, p.y);
}