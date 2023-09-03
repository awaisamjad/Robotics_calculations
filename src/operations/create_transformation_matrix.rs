use crate::print_operations_and_take_input;
use colored::Colorize;
use nalgebra::{Matrix3, Matrix4, Point3, Vector4};
use std::f64;
use std::io;

pub fn create_transformation_matrix()  {
    //~ Create base transformation matrix with all 0 values
    let mut transformation_matrix = Matrix4::new(
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    );
    loop {
        //~ Loop until user enters valid input
        println!("Enter values for the transformation matrix in the following format (Enter Quit to escape):");
        println!(
            "Rotation Matrix: r11,r12,r13,r21,r22,23,r31,r32,r33 \nExample: 1,0,0,0,1,0,0,0,1"
        );
        println!("Translation Vector: p1,p2,p3 \n Example: 1,2,3");

        let mut rotation_matrix_input = String::new(); //~ Empty string to store rotation matrix
        let mut translation_vector_input = String::new(); //~ Empty string to store translation vector

        //~ Read user input for rotation matrix
        io::stdin()
            .read_line(&mut rotation_matrix_input)
            .expect("Failed to read rotation matrix");

        //~ Turn the rotation matrix string into a vector of floats
        let rotation_matrix_values: Vec<f32> = rotation_matrix_input
            .split(',')
            .map(|val| {
                val.trim()
                    .parse::<f32>()
                    .expect("Invalid input for rotation matrix")
            })
            .collect();

        //~ If the user doesnt enter exactly 9 values, then give warning & restart the loop
        if rotation_matrix_values.len() != 9 {
            let error_message =
                format!("Warning: Please enter exactly 9 values for the rotation matrix.");
            println!("{}", error_message.red());
            continue;
        }

        //~ if user enters 'Quit' then present 2 options: exit program or go back to options
        if rotation_matrix_input.trim().eq_ignore_ascii_case("Quit") {
            println!("Do you want to:");
            println!("1. Quit the program");
            println!("2. Go back to the list of options");

            let mut user_choice = String::new(); //~ Empty string to store user choice
            io::stdin()
                .read_line(&mut user_choice)
                .expect("Failed to read user choice");

            match user_choice.trim() {
                "1" => {
                    println!("Exiting program."); //~ Exit program
                    break;
                }
                "2" => {
                    println!("Going back to list of options."); //~ Go back to list of options
                    print_operations_and_take_input();
                }
                _ => {
                    let error_message =
                        format!("Warning: Please enter either 1 or 2 for your choice.");
                    println!("{}", error_message.red());
                    continue;
                }
            }
        }

        //~ Read user input for translation vector
        io::stdin()
            .read_line(&mut translation_vector_input)
            .expect("Failed to read translation vector");

        //~ Turn the translation vector string into a vector of floats
        let translation_vector_values: Vec<f32> = translation_vector_input
            .split(',')
            .map(|val| {
                val.trim()
                    .parse::<f32>()
                    .expect("Invalid input for translation vector")
            })
            .collect();

        //~ If the user doesnt enter exactly 3 values, then give warning & restart the loop
        if translation_vector_values.len() != 3 {
            let error_message =
                format!("Warning: Please enter exactly 3 values for the translation vector.");
            println!("{}", error_message.red());
            continue; // Restart the loop
        }

        println!("Rotation Matrix Values: {:?}", rotation_matrix_values);
        println!("Translation Vector: {:?}", translation_vector_values);

        transformation_matrix.set_column(
            0,
            &Vector4::new(
                rotation_matrix_values[0],
                rotation_matrix_values[3],
                rotation_matrix_values[6],
                0.0,
            ),
        );
        transformation_matrix.set_column(
            1,
            &Vector4::new(
                rotation_matrix_values[1],
                rotation_matrix_values[4],
                rotation_matrix_values[7],
                0.0,
            ),
        );
        transformation_matrix.set_column(
            2,
            &Vector4::new(
                rotation_matrix_values[2],
                rotation_matrix_values[5],
                rotation_matrix_values[8],
                0.0,
            ),
        );
        transformation_matrix.set_column(
            3,
            &Vector4::new(
                translation_vector_values[0],
                translation_vector_values[1],
                translation_vector_values[2],
                1.0,
            ),
        );
        break;
    }
    println!("Matrix: {}", transformation_matrix);
    // ~ Loop at end of calulation to ask user if they want to perform operation on result, perform another operation or quit
    loop {
        println!(
            "Would you like to:\n
            1. Perform operation on result\n
            2. Perform another operation\n
            3. Quit"
        );

        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read user choice");

        match user_choice.trim() {
            "1" => {
                loop {
                    println!(
                    "What operation would you like to perform on the result?\n
                    1. Inverse\n
                    2. Multiply by another transformation matrix\n
                    3. Multiply by an inverse transformation matrix\n"
                    );

                    let mut user_choice = String::new();
                    io::stdin()
                        .read_line(&mut user_choice)
                        .expect("Failed to read user choice");

                    match user_choice.trim() {
                        "1" => {
                            println!("Performing inverse operation on result.");
                            println!("Matrix: {} ", transformation_matrix.try_inverse().unwrap());
                            break;
                        }

                        "2" => {
                            println!("Operation 2")
                        }

                        "3" => {
                            println!("Operation 3")
                        }

                        _ => {
                            println!("Invalid choice. Please select 1, 2, or 3.");
                        }
                    }
                } // Closing curly brace for the inner loop
            }
            "2" => {
                println!("Performing another operation.");
                print_operations_and_take_input();
                // Add your operation logic here
            }
            "3" => {
                println!("Exiting program.");
                break; // Exit the loop
            }
            _ => {
                println!("Invalid choice. Please select 1, 2, or 3.");
            }
        }
    }

}