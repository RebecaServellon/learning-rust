#![allow(unreachable_code, unused_labels)]

// Flow of control

fn main() {
    // If/Else
    // - the boolean condition doesn't need to be sorrounded by parenthesis
    // - all branches must return the same type

    println!("\nIf/Else");

    let n = 9;

    if n < 0 {
        print!("{} is negative", n);
    }
    else if n > 0 {
        print!("{} is positive", n);
    }
    else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        }
        else {
            println!(", and is a big number, halve the number");
            n / 2
        };

    println!("{} -> {}", n, big_n);

    // ________________________________________

    // Loop
    // Rust provides a loop keyword to indicate an infinite loop
    // - used for retrying an operation until it succeeds
    // - break statement to exit a loop anytime
    // - continue statemet to skip the rest of the iteration

    println!("\nLoop");

    let mut count = 0u32;
    println!("Let's count until infinity");

    loop {
        count += 1;

        if count == 3 {
            println!("Three...");
            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enoungh");

            // Exit this loop
            break;
        }
    }

    // _____________________________
    // Nesting and labels
    // I'st possible to break or continue outer loops when dealing with nested loops, but must be annotated with some 'label

    println!("\nNesting and labels");

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");
            // This would break only the inner loop
            // break;

            // This breaks the outer loop
            // When the outer loop terminates, both the inner and the outer loop will not continue
            break 'outer;
        }
        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    // _____________________________________
    // Returning from loops
    // Put the result value after the break and will be returned by the loop expression

    println!("\nReturning from loops");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("Result is {}", result);
    assert_eq!(result, 20);

    // While
    // - run a loop while a condition is true

    println!("\nWhile");


    // FizzBuzz
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0{
            println!("fizzbuzz");
        }
        else if n % 3 == 0 {
            println!("fizz");
        }
        else if n % 5 == 0 {
            println!("buzz");
        }
        else {
            println!("{}", n);
        }

        // Increment the counter
        n += 1;
    }

    // For loops

    println!("\nFor loops");


    // For and range
    // - iterate through an Iterator -> range notation a(inclusive)..b(exclusive) oor a(inclusive)..=b(inclusive)
    println!("\nFor and range");

    // FizzBuzz using for
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        }
        else if n % 5 == 0 {
            println!("buzz");
        }
        else {
            println!("{}", n);        
        }
    }

    // For and iterators
    println!("\nFor and iterators");

    // iter: borrows each element of the collection through each iteration. Thus leaving the collection untouched and available for reuse after the loop
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        // names.iter methos return an iterator that yields references to the elements of the vector
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    // into_iter: consumes the collection, so that the exact data is provided. Once the collections has been consumed it is no longer available for reuse
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // no longer available
    //println!("names {:?}", names);

    // _____________________________________________
    // iter_mut: this mutably bowwows each element of the collection, allowing for the collection to be modified in place
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names {:?}", names);


}
