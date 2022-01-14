fn main() {
    // Variable and Mutability
    // -----------------------
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(
        "The value of THREE_HOURS_IN_SECONDS is: {}",
        THREE_HOURS_IN_SECONDS
    );

    // Shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y is: {}", y);
    }

    println!("The value of y is: {}", y);

    // The 'let' keyword creates a new variable.
    // Helps us change the type of the value.
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // Data Types
    // ----------

    // Integer Types

    // | Length  | Signed | Unsigned |
    // | ------- | ------ | -------- |
    // | 8-bit   | i8     | u8       |
    // | 16-bit  | i16    | u16      |
    // | 32-bit  | i32    | u32      |
    // | 64-bit  | i64    | u64      |
    // | 128-bit | i128   | u128     |
    // | arch    | isize  | usize    |

    // Floating-Point Types

    // | Length  | Signed |
    // | ------- | ------ |
    // | 32-bit  | f32    |
    // | 64-bit  | f64    |

    // Numeric Operations

    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    println!("The value of quotient is: {}", quotient);
    println!("The value of floored is: {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);

    // Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("The value of t and f is: {}, {}", t, f);

    // Character Type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!(
        "The value of c, z, heart_eyedcat: {}, {}, {}",
        c, z, heart_eyed_cat
    );

    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("The value of tup is: {:#?}", tup);

    let five_hundred = tup.0; // tuple destructuring
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);

    // Array Type
    let months = [
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
    println!("The value of months is: {:#?}", months);

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of numbers is: {:#?}", numbers);
    println!("Accessing first element of numbers: {}", numbers[0]);
    println!("Accessing seconds element of numbers: {}", numbers[1]);

    let threes = [3; 5];
    println!("The value of threes is: {:#?}", threes);

}
