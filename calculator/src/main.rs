use std::env::args;

/// Calculator program to calculate using args
fn main() {
    let mut args = args();

    // nth removed the element accessed
    let first = args.nth(1).unwrap();
    // After accessing the second argument, the iterator's next element becomes the first
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    // convert string into number
    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    println!(
        "{} {} {} = {}",
        first_number,
        operator,
        second_number,
        operate(operator, first_number, second_number)
    );
}

/// perform operation on 2 numbers and return the result in f32 format
fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '/' => first_number / second_number,
        '*' => first_number * second_number,
        _ => panic!("Invalid Operator Used"),
    }
}
