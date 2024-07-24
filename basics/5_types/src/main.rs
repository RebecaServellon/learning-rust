// Types
// Rust provides several mechanisms to change or define the type pf primitive and user defined types


// Suppress all warnings from casts which overflow
#![allow(overflowing_literals)]

// Aliasing
// NanoSecond, Inch, and U64 are new names for u64
type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    // Casting
    println!("Casting");
    // Explicit type conversion - casting - is used the as keyword
    let decimal = 65.4321_f32;

    // Explicit conversion
    let integer = decimal as u8;
    // Error no implicit conversion
    // let integer:u8 = decimal;

    let character = integer as char; 
    // Error limitations in conversion rules
    // let character = decimal as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // Casting to unsigned types - u8, u16
    // If the value fits within the new type, it remains unchanged
    // If it doesn't fit, a modular operation adjusts the value to fit within the new type´s range
    println!("1000 as a u16 is {}", 1000 as u16); // Output 1000

    println!("1000 as a u8 is {}", 1000 as u8); // Output 232 (1000 % 256)

    // Casting to signed types - i8, i16
    // If the value fits within the new typw, it remains unchanged
    // If it doesn't, it is adjusted considering the two's complement representation
    println!("128 as a i16 is {}", 128 as i16); // Output 128

    println!("128 as a i8 is {}", 128 as i8); // Output -128

    // Casting from floating point to integer
    // Uses a saturating cast, values outside the bounds are adjusted to the maximum or minimum value of the new type
    // NaN is cast to 0
    println!("300.0 as u8 is {}", 300.0_f32 as u8); // Output 255

    println!("-100.0 as u8 is {}", -100.0_f32 as u8); // Output 0

    println!("NaN as u8 is {}", f32::NAN as u8); // Output 0

    // This behaviour incurs a small runtime cost and can be avoided with unsafe methods, however the results might overflow and return **unsound values**
    unsafe {
        // Output 300.0 as u8 is 44
        println!("300.0 as u8 is {}", 300.0_f32.to_int_unchecked::<u8>());

        // -100.0 as u8 is 156
        println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());

        // nan as u8 is 0
        println!("NAN as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }

    // _________________________________

    // Literals can be type annotated by adding the type as a suffix
    println!("\nLiterals");
    
    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // 'size_of_val' returns the size of a variable in bytes
    println!("Size of 'x' in bytes {}", std::mem::size_of_val(&x));
    println!("Size of 'y' in bytes {}", std::mem::size_of_val(&y));
    println!("Size of 'z' in bytes {}", std::mem::size_of_val(&z));
    println!("Size of 'i' in bytes {}", std::mem::size_of_val(&i));
    println!("Size of 'f' in bytes {}", std::mem::size_of_val(&f));


    // Inferencelooks at how the variable is used afterwars to infer its type
    println!("\nInference");

    let elem = 5u8;
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of vec
    vec.push(elem);
    // Now the compiler knows that `vec` is a vector of u8's (Vec<u8>)
    println!("{:?}", vec);

    // ___________________________________________
    // Aliasing
    // The type statement can be used to give a new name or and existing type
    println!("\nAliasing");

    let nanoseconds:NanoSecond = 5 as u64;
    let inches:Inch = 2 as U64;

    // Aliases are not new types
    println!("{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches, 
        nanoseconds + inches);





}
