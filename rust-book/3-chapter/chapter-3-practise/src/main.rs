fn main() {
    let celsius = fahrenheit_to_celsius(68.0);
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!(
        "{} degrees celsius is like saying {} degrees fahrenheit",
        celsius, fahrenheit
    );

    fibonacci(10);
}

fn celsius_to_fahrenheit(degrees: f64) -> f64 {
    degrees * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celsius(degrees: f64) -> f64 {
    (degrees - 32.0) * 5.0 / 9.0
}

fn fibonacci(cap: i32) {
    print!("Fibonacci sequence capped at {}: ", cap);
    let mut first_number = 0;
    let mut second_number = 1;
    let mut third_number = 2;

    while third_number < cap {
        if first_number == 0 && second_number == 1 {
            print!("{} {} ", first_number, second_number);
        }

        third_number = first_number + second_number;
        first_number = second_number;
        second_number = third_number;

        print!("{} ", third_number);
    }
}
