
fn main() {
    // Variable bindings can be annotate when declared

    let an_integer = 1u32;
    let copied_integer = an_integer;

    println!("An integer {:?}", copied_integer);

    // The compiler warns about unused variable bindings, these can be silenced by prefexing the variable name with an underscore
    // let unused_variable = 3u32; 
    let _unused_variable = 3u32;

    //_________________________________________

    // Mutability, by default variable bindings are immutable but can be overridden using mut
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation {}", mutable_binding);
    mutable_binding += 1;
    println!("After mutation {}", mutable_binding);

    // _immutable_binding += 1; Error

    // ________________________________________

    // Scope and shadowing
    let shadowed_binding = 1;

    {
        println!("Before being shadowed {}", shadowed_binding);
        let shadowed_binding = "abc";
        println!("After being shadowed in inner block {}", shadowed_binding);
    }
    println!("Outside inner block {}", shadowed_binding);

    let shadowed_binding = 2;
    println!("Shadowed in outer block {}", shadowed_binding);

    // _______________________________________
    // Declare first
    // The compiler forbids use of unintialized variables due to lead to undefined behaviour

    let another_binding:u32;
    //println!("Another binding {}", another_binding); Error
    another_binding = 1;
    println!("Another binding {}", another_binding);

    // ________________________________________
    // Freezing
    // When data is bound by the same name immutably, it also freezes
    // Frozen data can't be modified until the immutable binding goues out of scope

    let mut _mutable_integer = 7i32;

    {
        // Shadowing
        let _mutable_integer = _mutable_integer;
        // _mutable_integer goes out of scope
        // _mutable_integer = 50; Error
    }

    // _mutable_integer is not frozen in this scope
    _mutable_integer = 3;
    println!("Freezing example {}", _mutable_integer);
}
