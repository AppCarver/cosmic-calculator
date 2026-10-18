fn main() {
    let sum = add(5, 7);
    println!("The sum of 5 and 7 is: {}", sum);

    let result_add = calculate(12, 4, Operation::Add);
    println!("The result of the add calculation is: {}", result_add);

    let result_subtract = calculate(12, 4, Operation::Subtract);
    println!(
        "The result of the subtract calculation is: {}",
        result_subtract
    );

    let result_multiply = calculate(12, 4, Operation::Multiply);
    println!(
        "The result of the multiply calculation is: {}",
        result_multiply
    );

    let result_divide = calculate(12, 4, Operation::Divide);
    println!("The result of the divide calculation is: {}", result_divide);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn calculate(a: i32, b: i32, operation: Operation) -> i32 {
    match operation {
        Operation::Add => a + b,
        Operation::Subtract => a - b,
        Operation::Multiply => a * b,
        Operation::Divide => a / b,
    }
}
