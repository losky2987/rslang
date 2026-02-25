/// Das Problem tritt mit einem u64 nicht auf, da u64 Copy implementiert 
/// und somit kopiert wird, statt gemoved zu werden.
/// Wir könnten statt den vorgeschlagenen Änderungen 
/// auch #[derive(Clone)] zu #[derive(Copy, Clone)] ändern. 
/// Damit würde sich Foo im Hinblick auf moven so verhalten, wie u64.
/// Warum könnte es sinnvoll sein nicht immer einfach #[derive(Copy, Clone)], 
/// sondern je nach Anwendung auch nur #[derive(Clone)] zu verwenden?


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
    let foo2 = my_foo.clone();
    
    print_foo(my_foo);
    
    /* TODO: Hier fehlt etwas */
    my_foo = foo2;
    
    my_foo.val += 2;
    
    print_foo(my_foo);
}

