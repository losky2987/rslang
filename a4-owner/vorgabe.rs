/// Warum tritt das Problem nicht auf, wenn wir statt dem Struct `Foo` 
/// einfach u64 als Typ für `my_foo` nutzen (`let mut my_foo = 4;`)?
///
/// Können wir statt an den markierten Stellen etwas zu verändern auch 
/// etwas an einer nicht markierten Stelle ändern, damit sich das 
/// struct Foo im Hinblick auf das Problem wie ein einfacher u64 Wert verhält?



#[derive(Clone)]
struct Foo {
	val: u64
}

fn print_foo(my_foo: Foo) {
	println!("my_foo's value: {}", my_foo.val);
}

fn main() {
	let mut my_foo = Foo { val: 4 };
	my_foo.val += 2;
	
	/* TODO: Hier fehlt etwas */
	let myfoo2  = my_foo.clone();
	
	print_foo(my_foo);
	
	/* TODO: Hier fehlt etwas */

	my_foo = myfoo2;
	
	my_foo.val += 2;
	
	print_foo(my_foo);
}
