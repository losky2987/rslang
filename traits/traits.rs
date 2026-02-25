trait Greet {
    fn hello(&self);
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn hello(&self) {
        println!("Hello, I'm {}", self.name);
    }
}

fn main() {
    // Schritt 1
    let person = Person { name: String::from("ABC"), };
    person.hello();

    // Schritt 2: Traits describe behaviour but do not define a type
    // let greet: Greet = person; // Fehler
    // greet.hello(); 

    // Schritt 3: Borrowed Trait Object 
    let greet: &dyn Greet = &person;
    greet.hello(); 

    // Schritt 4: Owned Trait Object
   // let greet: Box<dyn Greet> = Box::new(person);
    greet.hello();
}
