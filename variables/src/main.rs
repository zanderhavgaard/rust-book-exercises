fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("const: {THREE_HOURS_IN_SECONDS}");

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("Inner scope x: {x}");
    }
    println!("The value of x: {x}");

    let spaces = "   ";
    // shadowing pervious var by creating new var with same name
    let spaces = spaces.len();
    println!("Number of spaces: {spaces}");

    // let mut spaces = "   ";
    // not allowed, as we are changing the type
    // spaces = spaces.len();

    let my_int = -3; // defaults i32
    let my_unsigned_int: u32 = 3;
    let my_signed_int: i32 = -3;

    println!(
        "my_int: {my_int}, my_unsigned_int: {my_unsigned_int}, my_signed_int: {my_signed_int}"
    );

    // allowed to use '_' to make numbers easier to read
    let _ = 1000 == 1_000;

    let _my_float = -2.0; // defaults to f64, all floats are signed
    let _my_f64: f64 = 2.0;
    let _my_f32: f32 = 2.0;

    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference_int = 95 - 4;
    let _difference_float = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3;

    // remainder
    let _remainder = 43 % 5;

    // bools
    let _true = true;
    let _false: bool = false;

    // 'char' character types, note the single quotes
    let _c = 'z';
    let _z: char = 'Z';
    let _heart_eyed_cat = '😻';

    // tuple
    let _typed_tup: (i32, f64, u8) = (500, 6.4, 1);
    let _tup = (500, 6.4, 1);

    // destructure tuple into variables
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");

    // access tuple elements by index
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;
    println!("{x}, {y}, {z}");

    // empty tuple is a unit
    let _unit = ();

    // array of int, fixed length == allocated to stack, not heap
    // length is implicit
    let _my_array = [1, 2, 3, 4, 5];

    // good usecase for arrays is static data with a fixed number of elements,
    // like the names of months as strings
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    // array with type annotation and explicit length
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    // initialize an array with the same value in all elements
    let _my_initialized_array = [0; 5];
    // will be [0,0,0,0,0]

    // access array elements by index
    let _first = my_array[0];
    let _second = my_array[1];
    let _third = my_array[2];
}
