fn main() {
    let x = 5;
    println!("The value of x is: {}", x + 2);

    //This is bound to crash the program
    //All the variables are immutable by default
    //x = 6;
    //println!("The value of x is: {x}");

    //Declaring mutables
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    //Immutables vs constants
    //Constants are always immutable
    //Can't use mut

    //TODO: Name all your constants in uppercase
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!(
        "The value of Three hours in seconds: {}",
        THREE_HOURS_IN_SECONDS
    );

    //Learning about Variable Shadowing
    let y = 12;
    //Using the let keyword effectively makes a new variable and doesn't make the previous variable mut
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    } // The Lifetime of innerscope is inside the curly braces

    println!("The value of y is: {y}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces: {}", spaces);

    //TODO: Data Types in Rust
    //Rust is a statically-typed language
    //It must know the type during compile time

    //When many types are possible, we must add type notation
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);

    //FIXME: Will throw errors
    // let guess = "42".parse().expect("Not a number!");
    // println!("{}", guess);

    //Scalar - Single Value
    // Integers
    // i32 - 32-bit Integer
    // u32 - usigned 32-bit integer
    //size of i and u can be 2^n
    // u8, u16, u32, u64, u128

    //Char
    let c = 'z';
    println!("{}", c);

    let z: char = 'e';
    println!("{}", z);

    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);
}
