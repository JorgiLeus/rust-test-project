use std::fmt::Debug;
use std::fmt;

fn main() {
    say_hello_world();
    introduce_yourself();
    calculate_x();
    formatted_prints();
    debug();
    display();
}

fn say_hello_world() {
    //Print text to console
    println!("Hello, world!");
}

fn introduce_yourself() {
    println!("I'm a Rustacean!");
}

fn calculate_x() {
    //Comments can be useful
    //Try changing the result
    let x = 5 + /*90 +*/ 5;
    println!("Is `x` 10 or 100? x = {}", x);
}

fn formatted_prints() {
    //The '{}' will be automatically replaced with any argument. The arguments will be stringified.
    println!("{} days", 31);

    //Positional arguments can be used here.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    //Or named arguments
    println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");

    //Special formatting can be specified after ':'.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    //You can right align text with a specified width.
    println!("{number:>width$}", number=1, width=6);

    //You can prefix numbers with zeroes.
    println!("{number:0>width$}", number=1, width=6);

    //Rust can check the correct amount of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    //println!("This struct `{}` won't print...", Structure(3));

    //Print 'Pi is roughly 3.142' by controlling the number of decimal places shown.
    //You can use let pi = 3.141592 as an estimate for pi.
    //You might find help here: https://doc.rust-lang.org/std/fmt/
    let pi = 3.141592;
    println!("Pi is roughly {number:.3}", number=pi);
}

fn debug() {
    // This structure cannot be printed either with `fmt::Display` or
    // with `fmt::Debug`.
    //struct UnPrintable(i32);

    // The `derive` attribute automatically creates the implementation
    // required to make this `struct` printable with `fmt::Debug`.
    #[derive(Debug)]
    struct DebugPrintable(i32);

    // Derive the `fmt::Debug` implementation for `Structure`. `Structure`
    // is a structure which contains a single `i32`.
    #[derive(Debug)]
    struct Structure(i32);

    // Put a `Structure` inside of the structure `Deep`. Make it printable
    // also.
    #[derive(Debug)]
    struct Deep(Structure);
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Now {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8
    }

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);
    println!("Peter's age: {} and name: {}", peter.age, peter.name);
}

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`.
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn display() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex { real: 3.3, imag: 7.2 };

    println!("Compare complex:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

    // Error. Both `Debug` and `Display` were implemented, but `{:b}`
    // requires `fmt::Binary` to be implemented. This will not work.
    // println!("What does Point2D look like in binary: {:b}?", point);
}