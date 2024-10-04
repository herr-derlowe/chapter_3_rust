pub fn if_condition(x: i32) {
    if x >= 0 {
        println!("x is equal or greater than zero")  ;
    } else {
        println!("x is less than zero")  ;
    }
}

pub fn loops(x: i32){
    let mut counter = 0;

    // Counts until double the value of x
    let result = loop {
        counter += 1;

        if counter == x {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

pub fn label_loops(){
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

pub fn while_loop(){
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

pub fn for_loop_array(x: &[i32]){
    for element in x {
        println!("Current value of x is {element}");
    }
}

pub fn for_loop_range(){
    let range: std::ops::Range<i32> = 1..6;
    for number in range.rev() {
        println!("Current number is: {number}");
    }
}