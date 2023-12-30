extern crate asalang;

use asalang::parser::{conditional_expression, Node}; // Import conditional_expression from the parser module

fn main() {
    let test_input = "5 > 3"; // Sample conditional expression
    match conditional_expression(test_input) {
        Ok((remaining_input, node)) => {
            println!("Parsed Node: {:?}", node);
            println!("Remaining Input: {:?}", remaining_input);
        },
        Err(e) => println!("Error: {:?}", e),
    }
}
