mod control;
mod fibonacci;

fn main() {
    let mut x = 5;
    println!("X is equal to {x}");

    x = 1;
    println!("X is equal to {x}");
    let y = "     ";
    let y = y.len();
    println!("Y has {y} number of spaces");

    // Variables
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("Sum: {sum}\nSubstraction: {difference}\nMultiplication: {product}\nDivision 1: {quotient}, Division 2: {truncated}\nRemainder: {remainder}");

    let array1 = [1, 2, 3, 4, 5];
    println!("{}", array1[remainder]);

    control::if_condition(-90);
    control::loops(4);

    control::label_loops();
    control::while_loop();
    control::for_loop_array(&[90, 80, 70, 60]);

    control::for_loop_range();
    
    for number in 0..10 {
        println!("The {}th Fibonacci number is: {}", number, fibonacci::fibonacci_practice(number));
    }
}
