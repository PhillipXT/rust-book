fn main() {
    mutable();
    shadowing();
}

fn mutable() {
    // Variables are immutable by default
    // Declare them with the 'mut' keyword to make them mutable
    println!("Mutable:");
    let mut x = 5;
    println!("\tThe value of x is: {x}");
    x = 6;
    println!("\tThe value of x is: {x}");
}

fn shadowing() {
    // Shadowing is a way to reassign a variable using the 'let' keyword
    let x = 5;
    let x = x + 1;

    println!("Shadowing:");
    {
        let x = x * 2;
        println!("\tThe value of x in the inner scope is {x}");
    }

    println!("\tThe value of x is: {x}");

    // We can change the type of the variable with shadowing
    let x = "Hello, World!";
    println!("\tThe value of x is: {x}");

    // We can reuse a variable name instead of coming up with new ones
    let spaces = "    ";
    let spaces = spaces.len();
    println!("\tThe values of spaces is: {spaces}");
}
