use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl Display for City {
    // f is a buffer
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};

        write!(f, "{}: {:.3}°{} {:.3}°{}",
                self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    // f is a buffer

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let rgb = (self.red as u32 * 65536) + (self.green as u32 * 256) + self.blue as u32;


        write!(f, "RGB ({red:02} {green:02} {blue:02}) 0x{colorCalculation:06X}",
            red = self.red,
            green = self.green,
            blue = self.blue,
            colorCalculation = rgb
        )
    }
}

fn main() {
    // Formatted print
    println!("Hello {}, world!", 66);

    // Positional arguments
    println!("Hello {1} {0}, world!", 66, "hello");

    // Named arguments
    println!("{subject} {verb} {object}",
            object = "to the world",
            subject = "I",
            verb = "say hello");

    // Printing with `{:?}` is similar to with `{}`
    println!("{:?} months in a year.", 12);

    // Rust also provides "pretty printing" with {:#?}
    let name = "Peter";
    let age = 27;
    let peter = Person {name, age};
    println!("{:#?}", peter);

    // fmt::Display is not implemented for generic containers. fmt::Debug must then be used for these generic cases.

    // ACTIVITY________________________________________
    // Rust even checks to make sure the correct number of arguments are used
    // Fix the issue (see FIXME) so that it runs without error...
    //println!("My name is {0}, {1} {0}", "Bond");
    // FIXME ^ Add the missing argument: "James"
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // ACTIVITY________________________________________
    // Try uncommenting the line that attempts to format the Structure struct (see TODO)
    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default
    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);
    // This will not compile because `Structure` does not implement
    // fmt::Display.
    //println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // ACTIVITY________________________________________
    // Add a println! macro call that prints: Pi is roughly 3.142 by controlling the number of decimal places shown. For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi. (Hint: you may need to check the std::fmt documentation for setting the number of decimals to display)

    let pi = 3.141592; 
    let decimals = 3;
    println!("Pi is roughly {pi:.decimals$}");

    //____________________________________________________________
    for city in [
        City {name: "Dublin", lat: 53.347778, lon: -6.259722},
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }

    // ACTIVITY________________________________________
    /*
    Add an implementation of the fmt::Display trait for the Color struct above so that the output displays as:

    RGB (128, 255, 90) 0x80FF5A
    RGB (0, 3, 254) 0x0003FE
    RGB (0, 0, 0) 0x000000
     */
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", color);
    }

}
