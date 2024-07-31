#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    // These likewise tie `u32` tuples to different names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

#[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(40)
}

fn main() {
    // _______________________________________________

    // match
    // pattern matching which can be used like C switch
    println!("\nMatch");


    let number = 13;
    println!("Tell me about {}", number);

    match number {
        // Match a single value
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime!"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    };

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
        // Ensure that all possible cases are being handled
    };

    println!("{} -> {}", boolean, binary);

    // Destructuring
    // A match block can destructure items in a variety of ways

    println!("\nDestructuring");
    println!("\nTuples");

    // Tuples
    let triple = (5, 6, 2);

    println!("Tell me about {:?}", triple);

    match triple {
        (0, y, z) => println!("First is 0, y is {:?} and z is {:?}", y, z),
        (1, ..) => println!("First is 1 and ther est doesn't matter"),
        (.., 2) => println!("Last is 2 and ther est doesn't matter"),
        (3, .., 4) => println!("First is 3, last is 4 and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    };

    // ______________________________________

    // Arrays/slices
    println!("\nArrays/slices");
    let array = [1, -2, 6];

    match array {
        [0, second, third] => 
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        [1, _, third] => 
            println!("array[0] = 1, array[2] = {} and array[1] was ignored", third),
        [-1, second, ..] =>
            println!("array[0] = -1, array[1] = {} and all the other ones were ignored", second),
        [3, second, tail @..] =>
            println!("array[0] = 3, array[1] = {} and the other elements were {:?}", second, tail),
        [first, middle @ .., last] => 
            println!("array[0] = {}, middle = {:?}, array[2] = {}", first, middle, last),
    };

    // _____________________________________
    // Enums
    println!("\nEnums");

    let color = Color::RGB(122, 17, 40);

    println!("What color is this?");

    match color {
        Color:: Red => 
            println!("The color is red!"),
        Color::Blue => 
            println!("The color is blue!"),
        Color::Green => 
            println!("The color is green"),
            Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    };

    // __________________________________

    // Pointers/ref
    // - dereferencing *
    // - destructuring &, ref, ref mut

    println!("\nPointers/ref");
    let reference = &4;

    match reference {
        &val => println!("Got a value via destructuring {:?}", val),
    };

    // To avoid &, you deference before matching
    match *reference {
        val => println!("Got a value via deferencing {:?}", val),
    };

    // Access the reference using &
    let _not_a_reference = 3;

    // Also you can use ref for this purpose
    let ref _is_a_reference = 3;

    // Refs can be retrieved by ref and ref mut
    let value = 5;
    let mut mut_value = 6;

    // Use ref keyword to creatre a reference
    match value {
        ref r => println!("Got a reference to a value {:?}", r),
    };

    // Use the ref mut similarly
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before wew can add anything to it
            *m += 10;
            println!("We added 10. Value is {:?}", m);
        }
    };

    // ___________________
    // Structs
    println!("\nStructs");

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } =>
            println!("First of x is 1, b = {}, y = {}", b, y),

        // You can destructure structs and rename the variables, order is not important
        Foo { y, ..} => 
            println!("y = {}, we don't care about x", y),
        
        // Error, pattern does not mention field x
        // Foo { y } => println!("y = {}", y),
    }

    let faa = Foo { x: (1, 2), y: 3 };

    // You don't need a match block to destructure strcuts
    let Foo { x: x0, y: y0 } = faa;
    println!("Outside x0 = {x0:?}, y0 = {y0}");

    // Destructuring works with nested structs as well
    struct Bar {
        foo: Foo,
    }

    let bar = Bar { foo: faa };
    let Bar { foo: Foo { x: nested_x, y: nested_y} } = bar;
    println!("Nested nested_x = {nested_x:?}, nested_y = {nested_y:?}");

    // _________________________________
    // Guards
    // A match guard can be added to filter the arm

    println!("\nGuards");

    let temperature = Temperature::Celsius(35);

    match temperature {
        Temperature::Celsius(t) if t > 30 => 
            // The if condition part is a guard
            println!("{}C is above 30 Celsius", t),
        Temperature::Celsius(t) => 
            println!("{}C is equal to or below 30 Celsius", t),
        Temperature::Fahrenheit(t) if t > 86 => 
            println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => 
            println!("{}F is equal to or below 86 Fahrenheit", t),
    }

    // The compiler won't take guard conditions into account when checking if all patterns are covered
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen"),
    }

    // ____________________
    // Binding

    println!("\nBinding");

    println!("Tell me what type of person you are");


    match age() {
        0 => 
            println!("I haven't celebrated my first birthfday yet"),
        n @ 1 ..=12 => 
            println!("I'm a child of age {:?}", n),
        n @ 13..=19 => 
            println!("I'm a teen of age {:?}", n),
        n => 
            println!("I'm an old person of age {:?}", n),
    }

    // You can also destructure enum variants, such as Option
    match some_number() {
        // Got Some variant, match if its value, bound to n, is equal to 42
        // The pattern Some(n @ 42) checks if the value inside Some is 42
        Some(n @ 42) => 
            println!("The answer is {}", n),
        Some(n) => 
            println!("Not interesting ... {}", n),
        _ => (),
    }
}