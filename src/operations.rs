use colored::Colorize;
use nalgebra::{Matrix3, Matrix4, Point3, Vector4};
use std::f64;
use std::io;

use crate::print_operations_and_take_input;
pub fn create_transformation_matrix() {
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
                    println!("Going back to list of options."); //TODO Go back to list of options
                    break;
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
    loop {
        println!(
            "Would you like to: 1. Perform operation on result 2. Perform another operation 3. Quit"
        );

        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read user choice");

        match user_choice.trim() {
            "1" => {
                println!("Performing operation on result.");
                // Add your operation logic here
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

pub fn inverse_transformation_matrix() {
    //~ Create base transformation matrix with all 0 values
    let mut transformation_matrix = Matrix4::new(
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    );
    loop {
        //~ Loop until user enters valid input
        println!("Enter values for the transformation matrix in the following format (Enter Quit to escape):");
        println!("Rotation Matrix: r11,r12,r13,r21,r22,23,r31,r32,r33");
        println!("Translation Vector: p1,p2,p3");

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
                    println!("Going back to list of options."); //TODO Go back to list of options
                    break;
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
        //~ Print the rotation matrix & translation vector
        println!("Rotation Matrix Values: {:?}", rotation_matrix_values);
        println!("Translation Vector: {:?}", translation_vector_values);

        //~ Calculate px
        let px = -((translation_vector_values[0] * rotation_matrix_values[0])
            + (translation_vector_values[1] * rotation_matrix_values[3])
            + (translation_vector_values[2] * rotation_matrix_values[6]));
        //~ Calculate py
        let py = -((translation_vector_values[0] * rotation_matrix_values[1])
            + (translation_vector_values[1] * rotation_matrix_values[4])
            + (translation_vector_values[2] * rotation_matrix_values[7]));
        //~ Calculate pz
        let pz = -((translation_vector_values[0] * rotation_matrix_values[2])
            + (translation_vector_values[1] * rotation_matrix_values[5])
            + (translation_vector_values[2] * rotation_matrix_values[8]));

        //~ Set the columns of the transformation matrix
        //& Ordering the values for rotation matrix already does the transpose
        transformation_matrix.set_column(
            0,
            &Vector4::new(
                rotation_matrix_values[0],
                rotation_matrix_values[1],
                rotation_matrix_values[2],
                0.0,
            ),
        );
        transformation_matrix.set_column(
            1,
            &Vector4::new(
                rotation_matrix_values[3],
                rotation_matrix_values[4],
                rotation_matrix_values[5],
                0.0,
            ),
        );
        transformation_matrix.set_column(
            2,
            &Vector4::new(
                rotation_matrix_values[6],
                rotation_matrix_values[7],
                rotation_matrix_values[8],
                0.0,
            ),
        );
        transformation_matrix.set_column(3, &Vector4::new(px, py, pz, 1.0));
        println!("Matrix: {}", transformation_matrix);
        break;
    }
}

pub fn rotation() {
    loop {
        let mut angle: f64 = 0.0;

        println!("Choose unit for the angle (degrees or radians, default is degrees):");
        let mut unit_choice = String::new();
        io::stdin()
            .read_line(&mut unit_choice)
            .expect("Failed to read unit choice");

        match unit_choice.trim() {
            "degrees" => {
                println!("Enter angle in degrees:");
                let mut angle_input = String::new();
                io::stdin()
                    .read_line(&mut angle_input)
                    .expect("Failed to read angle");

                let temp_angle: f64 = angle_input.trim().parse().expect("Invalid angle");

                angle = temp_angle.to_radians();
            }
            "radians" => {
                println!("Enter angle in radians:");
                let mut angle_input = String::new();
                io::stdin()
                    .read_line(&mut angle_input)
                    .expect("Failed to read angle");

                angle = angle_input.trim().parse().expect("Invalid angle");
            }
            _ => {
                println!("Invalid unit choice. Defaulting to degrees.");
            }
        }

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

        let y_rotation_matrix = Matrix3::new(
            angle.cos(),
            0.0,
            -angle.sin(),
            0.0,
            1.0,
            0.0,
            angle.sin(),
            0.0,
            angle.cos(),
        );

        let z_rotation_matrix = Matrix3::new(
            angle.cos(),
            -angle.sin(),
            0.0,
            angle.sin(),
            angle.cos(),
            0.0,
            0.0,
            0.0,
            1.0,
        );

        println!("What do you want to rotate?");
        println!("1. Rotation of a point about the X axis");
        println!("2. Rotation of a point about the Y axis");
        println!("3. Rotation of a point about the Z axis");
        println!("4. Rotation of a vector about the X axis");
        println!("5. Rotation of a vector about the Y axis");
        println!("6. Rotation of a vector about the Z axis");
        println!("7. Rotation of a frame about the X axis");
        println!("8. Rotation of a frame about the Y axis");
        println!("9. Rotation of a frame about the Z axis");
        println!("10. Exit"); // Add an option to exit the loop
        let mut user_choice = String::new(); //~ Empty string to store user choice
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read user choice");

        match user_choice.trim() {
            "1" => {
                println!("Enter the point in the following format: x,y,z");
                let mut point_input = String::new();
                io::stdin()
                    .read_line(&mut point_input)
                    .expect("Failed to read point");

                let point_values: Vec<f64> = point_input
                    .split(',')
                    .map(|val| val.trim().parse::<f64>().expect("Invalid input for point"))
                    .collect();

                if point_values.len() != 3 {
                    let error_message =
                        format!("Warning: Please enter exactly 3 values for the point.");
                    println!("{}", error_message.red());
                    continue;
                }

                let point = Point3::new(point_values[0], point_values[1], point_values[2]);

                let rotated_point = x_rotation_matrix * point;

                println!("Rotated point: {}", rotated_point);
            }
            "2" => {}
            "3" => {
                println!("Enter the point in the following format: x,y,z");
                let mut point_input = String::new();
                io::stdin()
                    .read_line(&mut point_input)
                    .expect("Failed to read point");

                let point_values: Vec<f64> = point_input
                    .split(',')
                    .map(|val| val.trim().parse::<f64>().expect("Invalid input for point"))
                    .collect();

                if point_values.len() != 3 {
                    let error_message =
                        format!("Warning: Please enter exactly 3 values for the point.");
                    println!("{}", error_message.red());
                    continue;
                }

                let point = Point3::new(point_values[0], point_values[1], point_values[2]);

                let rotated_point = z_rotation_matrix * point;

                println!("Rotated point: {}", rotated_point);
            }
            "4" => {}
            "5" => {}
            "6" => {}
            "7" => {}
            "8" => {}
            "9" => {}
            "10" => {
                println!("Exiting program."); //~ Exit program
                break;
            }
            _ => {
                let error_message =
                    format!("Warning: Please enter a number between 1 and 10 for your choice.");
                println!("{}", error_message.red());
                continue;
            }
        }
    }
}

pub fn rotate_about_y() {}

pub fn rotate_about_z() {}

pub fn translate() {}

pub fn scale() {}

pub fn shear() {}

pub fn forward_kinematics_via_dh_parameters() {}

pub fn inverse_kinematics() {}
