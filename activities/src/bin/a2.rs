// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

// * Use a function to add two numbers together
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// * Use a function to display the result

fn display(a: i32) {
    // Print the results to the terminal
    println!("{:?}", a);
}

// * Use the "{:?}" token in the println macro to display the result
fn main() {
    // Define num_1 and num_2
    let num_1 = 8;
    let num_2 = 9;
    
    // Run the add function with num_1 and num_2
    let result = add(num_1, num_2);
    display(result);
}





