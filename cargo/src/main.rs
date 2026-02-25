use rand::Rng; // Import aus der Crate

fn main() {
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen_range(1..=6);
    println!("Gewürfelt: {x}");
}
