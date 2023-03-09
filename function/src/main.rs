fn main() {
    println!("Hello, world!");
    another_function(-5, 'h');

    let m = add();
    println!("The value of m is: {m}");

    let n = sub();
    println!("The value of n is: {n}");
}

// Paramterized Function
fn another_function(x: i32, unit_label: char) {
    let h = {
        let x = 3;
        x + 1 // never add a semicolon here, it will change it from an expression to a statement
    };
    println!("Hello I am a function!");
    println!("The measurement is: {x}{unit_label}");
    println!("The value of y is: {h}");
}   

//function with return values
fn add() -> i32 {
    5
}

//Function using the return keyword
fn sub() -> i32 {
    return -5;
}
