/*
Custom types
- struct: tuple, classic C struct, unit structs for generics

- enum
*/ 

#![allow(dead_code)]

// Structs 

#[derive(Debug)]
struct Person {
    name: String, 
    age: u8,
}

// Unit struct
struct Unit;

// Tuple struct
struct Pair(i32, f32);

// Struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle{ top_left, bottom_right} = rectangle;
    let height = (top_left.y - bottom_right.y).abs();
    let width = (bottom_right.x - top_left.x).abs();
    height * width 
}

fn square(point: Point, value: f32) -> Rectangle {
    let top_left = Point { x: point.x, y: point.y };
    let bottom_right = Point { x: point.x + value, y: point.y + value};
    Rectangle {top_left, bottom_right} 
}

// _____________________________________________

// Enums
// Allows the creation of a type which may be one of a few differet variants.Any variant which is valid as a struct is also valid in a enum

// Enum to clasify web event
enum WebEvent {
    // An enum variant may be unit-like
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64},
}

// A function which takes a 'WebEvent' enum as an argument and returns nothing
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(c) => println!("pressed '{}'", c),
        WebEvent::Paste(s) => println!("pasted \"{}\"", s),
        WebEvent::Click {x, y} => {
            println!("clicked at x = {}, y = {}", x, y);
        },
    }
}

// Type aliases -  if the enum's name is too long or too generic
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

// The most common place you'll see this is in impl blocks using the Self alias
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

// use - manual scope is not needed
enum Stage {
    Beginner, 
    Advanced
}

// c-like 
enum Number {
    Zero,
    One, 
    Two,
}

// Constants
// - const - unchangeable value
// - static - possibly mutable variable with static lifetime

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person {name, age};
    println!("{:?}", peter);

    // Instantiate a Point
    let point: Point = Point{ x: 10.3, y: 0.4 };
    let another_point: Point = Point{ x: 5.2, y: 0.2 };
    println!("Point coordinates: ({}, {})", point.x, point.y);
    println!("Another point coordinates: ({}, {})", another_point.x, another_point.y);

    let bottom_right = Point{ x: 5.5, ..another_point };
    println!("Second point coordinates: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a let binding
    let Point { x: left_edge, y: top_edge} = point;
    println!("Binding a point: x is {} and y is {}", left_edge, top_edge);

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Unit struct
    let _unit = Unit;

    // Tuple struct
    let pair = Pair(1, 0.1);
    println!("Pair contains {:?} and {:?}", pair.0, pair.1);
    let Pair(integer, decimal) = pair;
    println!("Binding... Pair contains {:?} and {:?}", integer, decimal);

    // Activities...
    // 1. Add a function rect_area which calculates the area of a Rectangle (try using nested destructuring).
   
    println!("The area of the rectangle is {}", rect_area(_rectangle));


    // 2. Add a function square which takes a Point and a f32 as arguments, and returns a Rectangle with its top left corner on the point, and a width and height corresponding to the f32.
    let square_rect = square(point, 3.0);
    println!("The rectangle given a point and a value is {:?}", square_rect);



    // Enums ____________________
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click {x: 20, y: 80};
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    // Aliases
    let _abc = Operations::Add;

    // use
    use crate::Stage::{ Beginner, Advanced };
    // Equivalent to Stage::Beginner
    let stage = Beginner;

    match stage {
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects"),
    }

    // c-like
    // enums can be cast as integers
    println!("zero is {}", Number::Zero as i32);

    // Constants
    let n = 16;
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
}
