use crate::messages::messages::{error_message, highlighted_message, ordered_list_message, success_message};
use crate::print_operations_and_take_input;
use colored::Colorize;
use nalgebra::{DMatrix, Matrix3x1};
use nalgebra::{Matrix3, Matrix4, Point3, Vector4};
use regex::Regex;
use std::io;

pub fn create_transformation_matrix() {
    loop {
        //~ Loop until user enters valid input
        println!("Enter values for the transformation matrix in the following format (Enter Quit to escape):");
        highlighted_message(
            "Rotation Matrix: r11,r12,r13,r21,r22,23,r31,r32,r33 - Example: 1,2,3,4,5,6,7,8,9",
        );
        highlighted_message("Translation Vector: p1,p2,p3 - Example: 10,11,12");

        let mut rotation_matrix_input = String::new(); //~ Empty string to store rotation matrix
        let mut translation_vector_input = String::new(); //~ Empty string to store translation vector

        //~ Read user input for rotation matrix
        io::stdin()
            .read_line(&mut rotation_matrix_input)
            .expect("Failed to read rotation matrix");

        //~ if user enters 'Quit' then present 2 options: exit program or go back to options
        if rotation_matrix_input.trim().eq_ignore_ascii_case("Quit") {
            ordered_list_message(
                "Do you want to:\n1. Quit the program\n2. Go back to the list of options",
            );

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

        //~ Turn the rotation matrix string into a vector of floats
        let rotation_matrix_values: Vec<f32> = rotation_matrix_input
            .split(',')
            .map(|val| {
                val.trim().parse::<f32>().unwrap_or_else(|_| {
                    println!("Invalid input for rotation matrix. Please enter a number.");
                    0.0 // You can choose a default value here if needed.
                })
            })
            .collect();

        //~ If the user doesnt enter exactly 9 values, then give warning & restart the loop
        if rotation_matrix_values.len() != 9 {
            let error_message =
                format!("Warning: Please enter exactly 9 values for the rotation matrix.");
            println!("{}", error_message.red());
            continue;
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
        //~ Turns the rotation matrix values into a 3x3 matrix
        let rotation_matrix_values_new = Matrix3::from_row_slice(&rotation_matrix_values);
        //~ Turns the translation vector values into a 3x1 matrix
        let translation_vector_values_new = Matrix3x1::from_row_slice(&translation_vector_values);

        println!("Rotation Matrix Values: {}", rotation_matrix_values_new);
        println!("Translation Vector: {}", translation_vector_values_new);

        //~ Create transformation matrix
        let transformation_matrix = TransformationMatrix {
            rotation_matrix: rotation_matrix_values_new,
            translation_matrix: translation_vector_values_new,
        };
        println!("Matrix: {}", transformation_matrix.transformation_matrix());
        //     break;
        // }

        // // ~ Loop at end of calulation to ask user if they want to perform operation on result, perform another operation or quit
        // loop {
        ordered_list_message("Would you like to:\n1. Perform operation on result\n2. Go back to initial operations\n3. Quit");

        let mut new_user_choice = String::new();
        io::stdin()
            .read_line(&mut new_user_choice)
            .expect("Failed to read user choice");

        match new_user_choice.trim() {
            "1" => loop {
                ordered_list_message("What operation would you like to perform on the result?\n1. Inverse\n2. Multiply by another transformation matrix\n3. Multiply by an inverse transformation matrix\n");

                let mut user_choice = String::new();
                io::stdin()
                    .read_line(&mut user_choice)
                    .expect("Failed to read user choice");

                match user_choice.trim() {
                    "1" => {
                        println!("Performing inverse operation on result.");
                        println!(
                            "Matrix: {} ",
                            transformation_matrix.inverse_transformation_matrix()
                        );
                        break;
                    }

                    "2" => {
                        println!("Operation 2")
                    }

                    "3" => {
                        println!("Operation 3")
                    }

                    _ => {
                        error_message("Invalid choice. Please select 1, 2, or 3.");
                    }
                }
            },
            "2" => {
                println!("Going back to initial operations.");
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

//~ takes the transformation matrix and returns the inverse transformation matrix
fn inverse_on_transformation_matrix(mut transformation_matrix: Matrix4<f32>) -> Matrix4<f32> {
    let rotation_matrix: Matrix3<f32> = transformation_matrix.fixed_view::<3, 3>(0, 0).transpose();
    let translation_vector = transformation_matrix.fixed_view::<3, 1>(0, 3);
    let px = -((translation_vector[0] * rotation_matrix[0])
        + (translation_vector[1] * rotation_matrix[3])
        + (translation_vector[2] * rotation_matrix[6]));

    let py = -((translation_vector[0] * rotation_matrix[1])
        + (translation_vector[1] * rotation_matrix[4])
        + (translation_vector[2] * rotation_matrix[7]));

    let pz = -((translation_vector[0] * rotation_matrix[2])
        + (translation_vector[1] * rotation_matrix[5])
        + (translation_vector[2] * rotation_matrix[8]));

    transformation_matrix.set_column(
        0,
        &Vector4::new(
            rotation_matrix[0],
            rotation_matrix[1],
            rotation_matrix[2],
            0.0,
        ),
    );
    transformation_matrix.set_column(
        1,
        &Vector4::new(
            rotation_matrix[3],
            rotation_matrix[4],
            rotation_matrix[5],
            0.0,
        ),
    );
    transformation_matrix.set_column(
        2,
        &Vector4::new(
            rotation_matrix[6],
            rotation_matrix[7],
            rotation_matrix[8],
            0.0,
        ),
    );
    transformation_matrix.set_column(3, &Vector4::new(px, py, pz, 1.0));
    transformation_matrix
}

struct TransformationMatrix {
    rotation_matrix: Matrix3<f32>,
    translation_matrix: Matrix3x1<f32>,
}
impl TransformationMatrix {
    fn create_rotation_matrix(&self) -> Matrix3<f32> {
        self.rotation_matrix
    }

    fn create_translation_matrix(&self) -> Matrix3x1<f32> {
        self.translation_matrix
    }

    fn transformation_matrix(&self) -> Matrix4<f32> {
        Matrix4::new(
            self.rotation_matrix[0],
            self.rotation_matrix[3],
            self.rotation_matrix[6],
            self.translation_matrix[0],
            self.rotation_matrix[1],
            self.rotation_matrix[4],
            self.rotation_matrix[7],
            self.translation_matrix[1],
            self.rotation_matrix[2],
            self.rotation_matrix[5],
            self.rotation_matrix[8],
            self.translation_matrix[2],
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }

    fn inverse_px(&self) -> f32 {
        -((self.translation_matrix[0] * self.rotation_matrix[0])
            + (self.translation_matrix[1] * self.rotation_matrix[1])
            + (self.translation_matrix[2] * self.rotation_matrix[2]))
    }

    fn inverse_py(&self) -> f32 {
        -((self.translation_matrix[0] * self.rotation_matrix[3])
            + (self.translation_matrix[1] * self.rotation_matrix[4])
            + (self.translation_matrix[2] * self.rotation_matrix[5]))
    }

    fn inverse_pz(&self) -> f32 {
        -((self.translation_matrix[0] * self.rotation_matrix[6])
            + (self.translation_matrix[1] * self.rotation_matrix[7])
            + (self.translation_matrix[2] * self.rotation_matrix[8]))
    }

    fn inverse_transformation_matrix(&self) -> Matrix4<f32> {
        Matrix4::new(
            self.rotation_matrix[0],
            self.rotation_matrix[1],
            self.rotation_matrix[2],
            self.inverse_px(),
            self.rotation_matrix[3],
            self.rotation_matrix[4],
            self.rotation_matrix[5],
            self.inverse_py(),
            self.rotation_matrix[6],
            self.rotation_matrix[7],
            self.rotation_matrix[8],
            self.inverse_pz(),
            0.0,
            0.0,
            0.0,
            1.0,
        )
    }
}
