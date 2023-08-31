use std::collections::BTreeMap;
use std::{f64, io};
mod operations;
use colored::Colorize;
use nalgebra::{Matrix2, Matrix3, Point3, Vector4};
use rusymbols::{core, Expression}; //~ gives colours to text in terminal
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
                println!(
                    "You selected operation: {}",
                    list_of_operations.get(&choice_of_operation).unwrap()
                );
                match choice_of_operation {
                    1 => operations::create_transformation_matrix(),
                    2 => operations::inverse_transformation_matrix(),
                    3 => operations::rotation(),
                    4 => operations::translate(),
                    5 => operations::scale(),
                    6 => operations::shear(),
                    7 => operations::forward_kinematics_via_dh_parameters(),
                    8 => operations::inverse_kinematics(),
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
    let mut angle: f64 = 30.0;
    angle = angle.to_radians();
    let x_rotation_matrix = Matrix3::new(
        1.0,
        0.0,
        0.0,
        0.0,
        angle.cos(),
        -angle.sin(),
        0.0,
        angle.sin(),
        angle.cos(),
    );
    let pointlist = vec![0.0, 2.0, 0.0];
    let point = Point3::new(pointlist[0], pointlist[1], pointlist[2]);
    println!("{}", x_rotation_matrix * point);
}

fn main() {
    print_operations_and_take_input();
    // test();
}
