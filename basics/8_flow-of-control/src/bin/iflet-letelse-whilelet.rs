use std::str::FromStr;

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');

    // Ensure both elements exist
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };

    // Attempt to parse the first element
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };
    (count, item)
}

fn main() {
    // If let
    println!("\nIf let");
    // If let allows various failure options to be specified
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The if let construct reads if let destructures number into Some(i), evaluate the block
    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    // If you need to specify a failure, use an else
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        // Destructure failure
        println!("Didn't match a number. Let's go with a letter");
    }

    // Provide an altered failing condition
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
        // Destructure failed. Evaluate an else if condition to see if the alternate failure branch should be taken
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default
        println!("I don't like letters. Let's go with an emoticon");
    }

    // If let can be used to match any enum value
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // Variable b does not match, it will print nothing
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works for if let
    if let Foo::Qux(_value @ 100) = c {
        println!("c is one hundred");
    }

    // Another benefit is that if else allows us match non-parameterized enum variants. This is true even in cases where the enum doesn't implement or derive PartialEq
    // Let else
    // A refutable pattern can match and bind variables in the sorrounding scope like a normal let, or else diverse when the pattern doesn't match
    println!("\nLet else");

    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));

    // While let
    // Similar to if let, while let can make awkward match sequences more tolerable
    println!("\nWhile let");

    let mut optional = Some(0);

    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("i is {:?}. Try again!", i);
                    optional = Some(i + 1);
                }
            },
            // Why should this be required? There must be a better way!
            _ => { break; }
        }
    }

    // Using while let makes this sequence much nicer
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("i is {:?}. Try again!", i);
            optional = Some(i + 1);
        }
        // Doesn't require explicitly handling the failing case

        // If let had additional optional else/else if clauses. While let does not have these
    }

    
}