fn main() {
    // The values in a tuple need not share the same type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //The variable binds to the entire tuple, so in order
    //to get individual elements out, we destructure it
    //via pattern matching
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
