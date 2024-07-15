/*
Primitives

Scalar
- signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
- unsigned integers: u8, u16, u32, u128 and usize (pointer size)
- floating point: f32, f64
- char unicode 4 byes
- bool: true or false
- unit type (): empty tuple

Compound
- arrays
- tuples

*/
use std::any::type_name;
use std::fmt::{self, Formatter, Display};
use std::mem;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

// Tuples can be used as function arguments and as return values
fn reverse(tuple_of_tuples: ((u8, u16, u32), (u64, i8), i16)) -> (i16, (u64, i8), (u8, u16, u32)) {
    // Bind the members to variables
    let (first, middle, last) = tuple_of_tuples;
    (last, middle, first)
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);


impl Display for Matrix {
    // f is a buffer
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        write!(f, "Matrix:\n( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let Matrix(first, swapp_a, swap_b, last) = matrix;
    Matrix(first, swap_b, swapp_a, last)
}

// This function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice {} - Last element of the slice {}", slice[0], slice[slice.len() - 1]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // Types annotation are used for clarity, safety and documentation
    // 1- Without/type inference
    let x = 42;
    println!("The value of x is: {} with type {}", x, type_of(&x));

    // 2- Type annotation
    let x: i32 = 42;
    println!("The value of x is: {} with type {}", x, type_of(&x));

    // 3- Type annotations in functions
    let result = sum(10, 20);
    println!("The result is: {}", result);

    // 4- Type annotations for complex variables
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("The vector is: {:?} with type {}", numbers,  type_of(&numbers));

    // Tuples
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    // Long tuples +12 elements cannot be printed
    println!("Tuple of tuples: {:?}", tuple_of_tuples);
    println!("Tuple indexing: {}", tuple_of_tuples.0 .0);
    println!("Tuple of tuples reversed: {:?}", reverse(tuple_of_tuples));


    /* 
    //Activities
    1. Recap: Add the fmt::Display trait to the Matrix struct in the above example, so that if you switch from printing the debug format {:?} to the display format {}, you see the following output:
    ( 1.1 1.2 )
    ( 2.1 2.2 )
    */
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("{}", matrix);

    /*
    2- Add a transpose function using the reverse function as a template, which accepts a matrix as an argument, and returns a matrix in which two elements have been swapped. For example:
    Matrix:
    ( 1.1 1.2 )
    ( 2.1 2.2 )
    Transpose:
    ( 1.1 2.1 )
    ( 1.2 2.2 )
    */

    println!("Transpose:\n{}", transpose(matrix));

    // Arrays and Slices
    // The key difference between these two is that slices lenght is not known at compile time. Slices can be used to borrow a section of an array and have the type signature &[T]
    // Fixed size array
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("First array: {:?}", xs);

    // All elements initialized to the same value
    let ys: [i32; 50] = [10; 50];

    println!("Second array: {:?}", ys);
    println!("Second array's lenght: {}", ys.len());
    println!("Array occupies {} bytes", mem::size_of_val(&ys));

    // Arrays can be automatically borrowed as slices
    println!("Borrow the whole array as a slice");
    analyze_slice(&xs);

    // Section of the array [starting_index .. ending_index]
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);

    // Empty slice
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);

    // Array methods: .get() and .expect()
    // Out of bound indexing on array causes compile time error
    // Out of bound indexing on slice causes runtime error
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}
