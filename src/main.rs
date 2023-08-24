use std::collections::BTreeMap;
use std::io;
mod operations;
use nalgebra::{Matrix2, Vector4};
use rusymbols::{core, Expression};
use colored::*; //? gives colours to text in terminal
fn print_operations_and_take_input() {
    let mut list_of_operations: BTreeMap<u8, String> = BTreeMap::new();
    list_of_operations.insert(0, "Create Transformation Matrix".to_string());
    list_of_operations.insert(1, "Inverse Transformation Matrix".to_string());
    list_of_operations.insert(2, "Rotate about x".to_string());
    list_of_operations.insert(3, "Rotate about y".to_string());
    list_of_operations.insert(4, "Rotate about z".to_string());
    list_of_operations.insert(5, "Translate".to_string());
    list_of_operations.insert(6, "Scale".to_string());
    list_of_operations.insert(7, "Shear".to_string());
    list_of_operations.insert(8, "Forward Kinematics via DH Parameters".to_string());
    list_of_operations.insert(9, "Inverse Kinematics".to_string());

    let number_of_operations: u8 = list_of_operations.len() as u8;

    loop {
        println!("List of Operations:");

        for (idx, operation) in list_of_operations.iter() {
            println!("{}. {}", idx + 1, operation);
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
                println!(
                    "You selected operation: {}",
                    list_of_operations.get(&choice_of_operation).unwrap()
                );
                match choice_of_operation {
                    1 => operations::create_transformation_matrix(),
                    2 => operations::inverse_transformation_matrix(),
                    3 => operations::rotate_about_x(),
                    4 => operations::rotate_about_y(),
                    5 => operations::rotate_about_z(),
                    6 => operations::translate(),
                    7 => operations::scale(),
                    8 => operations::shear(),
                    9 => operations::forward_kinematics_via_dh_parameters(),
                    10 => operations::inverse_kinematics(),
                    _ => println!("Invalid input. Please enter a number between 1 and {}.", number_of_operations+1),
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
    // let x = Expression::new_var("x");
    // // let mut matrix = Matrix2::new(
    // //     x, 2.0,
    // //     3.0, 4.0,
    // // );
    // let x = Expression::new_var("x"); // new 'x' variable creation
    // let two = Expression::new_val(2.0); // new 2.0 value creation
    // let expr = Expression::new(x, two, core::Actions::Mul);
    // println!("{}", expr);
}

fn main() {
    print_operations_and_take_input();
    // test();
}
