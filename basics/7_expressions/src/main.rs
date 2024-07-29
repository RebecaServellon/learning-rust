
//  Expressions

fn main() {
    // Variable binding
    let x = 5;

    // Expression ;
    // Explicitly ignoring the result
    let _ = x;
    let _ = x + 1;
    let _ = 15;

    // Blocks are expressions too, so they can be used as values in assignments
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to y
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon supresses this expression and () is assigned to z
        let _ = 2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

}
