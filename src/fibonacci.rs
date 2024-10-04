pub fn fibonacci_practice(n : i32) -> i32{
    // let mut counter : i32= 0;
    // let mut fibonacci_number: i32 = 0;
    
    // let mut previous_two: [i32; 2] = [0, 1];
    // while counter <= n {
    //     fibonacci_number = previous_two[0] + previous_two[1];
    //     if counter == 0 ||counter < 2 {
    //         counter += 1;
    //         continue;
    //     }
    //     previous_two[0] = previous_two[1];
    //     previous_two[1] = fibonacci_number;

    //     counter += 1;
    // }
    // println!("The {}th Fibonacci number is: {}", n, fibonacci_number);

    // With recursiveness
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci_practice(n-1) + fibonacci_practice(n-2);
    }
}