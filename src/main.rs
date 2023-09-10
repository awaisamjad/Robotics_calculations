use colored::Colorize;
use nalgebra::{Matrix3, Matrix3x1, Matrix4, Vector4};
use regex::Regex;
use std::collections::BTreeMap;
use std::io;
mod operations {
    pub mod create_transformation_matrix;
    pub mod forward_kinematics_via_dh_parameters;
    pub mod inverse_kinematics;
    pub mod inverse_transformation_matrix;
    pub mod rotation;
    pub mod scale;
    pub mod shear;
    pub mod translate;
}
use crate::operations::create_transformation_matrix::create_transformation_matrix;
use crate::operations::forward_kinematics_via_dh_parameters::forward_kinematics_via_dh_parameters;
use crate::operations::inverse_kinematics::inverse_kinematics;
use crate::operations::inverse_transformation_matrix::inverse_transformation_matrix;
use crate::operations::rotation::rotation;
use crate::operations::scale::scale;
use crate::operations::shear::shear;
use crate::operations::translate::translate;
mod messages {
    pub mod messages;
}

use crate::messages::messages::{
    error_message, information_message, success_message, warning_message,
};

fn print_operations_and_take_input() {
    let mut list_of_operations: BTreeMap<u8, String> = BTreeMap::new();
    list_of_operations.insert(1, "Create Transformation Matrix".to_string());
    list_of_operations.insert(2, "Inverse Transformation Matrix".to_string());
    list_of_operations.insert(3, "Rotation".to_string());
    list_of_operations.insert(4, "Translate".to_string());
    list_of_operations.insert(5, "Scale".to_string());
    list_of_operations.insert(6, "Shear".to_string());
    list_of_operations.insert(7, "Forward Kinematics via DH Parameters".to_string());
    list_of_operations.insert(8, "Inverse Kinematics".to_string());

    let number_of_operations: u8 = list_of_operations.len() as u8;

    loop {
        println!("List of Operations:");

        for (idx, operation) in list_of_operations.iter() {
            println!("{}. {}", idx, operation);
        }
        println!("Enter the operation you would like by index (Enter Quit to escape):");

        let mut user_operation = String::new();

        io::stdin()
            .read_line(&mut user_operation)
            .expect("Failed to read operation");

        let user_operation = user_operation.trim();

        if user_operation.eq_ignore_ascii_case("Quit") {
            println!("Exiting program.");
            break;
        }

        if let Ok(choice_of_operation) = user_operation.parse::<u8>() {
            if choice_of_operation >= 1 && choice_of_operation <= number_of_operations {
                // println!(
                //     "You selected operation: {}",
                //     list_of_operations.get(&choice_of_operation).unwrap()
                // );
                success_message("You selected operation: ");
                print!("{}\n", list_of_operations.get(&choice_of_operation).unwrap());
                match choice_of_operation {
                    1 => create_transformation_matrix(),
                    2 => inverse_transformation_matrix(),
                    3 => rotation(),
                    4 => translate(),
                    5 => scale(),
                    6 => shear(),
                    7 => forward_kinematics_via_dh_parameters(),
                    8 => inverse_kinematics(),
                    _ => println!(
                        "Invalid input. Please enter a number between 1 and {}.",
                        number_of_operations + 1
                    ),
                }
                break;
            } else {
                let error_message = format!(
                    "Invalid input. Please enter a number between 1 and {}.",
                    number_of_operations
                );
                println!("{}", error_message.red());
            }
        } else {
            let error_message = format!(
                "Invalid input. Please enter a number between 1 and {}.",
                number_of_operations
            );
            println!("{}", error_message.red());
        }
    }
}

fn test() {
    // Create a new Regex for validation
    let pattern = r"^\d+(\.\d+)?(,\d+(\.\d+)?){8}$";
    let regex = Regex::new(pattern).unwrap();

    // Prompt the user for input
    loop {
        println!("Please enter a list of 9 numbers (integers or floats) separated by commas:");

        // Read a line of input from the user
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim leading and trailing whitespace from the input
        let input = input.trim();

        // Check if the input matches the pattern
        if regex.is_match(input) {
            println!("Input is in the correct format.");
            break; // Exit the loop when the input is correct
        } else {
            println!("Input is not in the correct format.");
        }
    }
}

fn main() {
    print_operations_and_take_input();
    // test();
    
}
