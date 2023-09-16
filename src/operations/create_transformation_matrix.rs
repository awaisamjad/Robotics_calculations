use crate::messages::messages::{error_message, highlighted_message, ordered_list_message};
use crate::{initial_operations, exit};
use colored::Colorize;
use nalgebra::Matrix3x1;
use nalgebra::{Matrix3, Matrix4, Vector4};

use std::io;


pub fn create_transformation_matrix() {
    loop {
        //~ Loop until user enters valid input
        println!("Enter values for the transformation matrix in the following format (Enter Quit to escape):");
        highlighted_message(
            "Rotation Matrix: r11,r12,r13,r21,r22,23,r31,r32,r33 - Example: 1,2,3,4,5,6,7,8,9",
        );
        highlighted_message("Translation Vector: p1,p2,p3 - Example: 10,11,12");

        //~ Turns the rotation matrix values into a 3x3 matrix
        let rotation_matrix_values_option = read_rotation_matrix();
        // let rotation_matrix_values_new = Matrix3::from_row_slice(&rotation_matrix_values);
        let rotation_matrix_values = match rotation_matrix_values_option {
            Some(values) => values,
            None => {
                // Handle the case where "Quit" was chosen or an invalid input occurred.
                // You can decide whether to exit or go back to options.
                break; // Call the exit_with_cleanup function
            }
        };
        let rotation_matrix_values_new = Matrix3::from_row_slice(&rotation_matrix_values);
        //~ Turns the translation vector values into a 3x1 matrix
        let translation_vector_values = read_translation_vector();
        let translation_vector_values_new = Matrix3x1::from_row_slice(&translation_vector_values);

        println!("Rotation Matrix Values: {}", rotation_matrix_values_new);
        println!("Translation Vector: {}", translation_vector_values_new);

        //~ Create transformation matrix
        let transformation_matrix = TransformationMatrix {
            rotation_matrix: rotation_matrix_values_new,
            translation_matrix: translation_vector_values_new,
        };
        println!("Transformation Matrix: {}", transformation_matrix.transformation_matrix());
        
        //~ Ask user if they want to perform operations on the matrix or go back to initial operations
        ordered_list_message("Would you like to:\n1. Perform operation on result\n2. Go back to initial operations\n3. Quit");

        let mut new_user_choice = String::new();
        io::stdin()
            .read_line(&mut new_user_choice)
            .expect("Failed to read user choice");


        match new_user_choice.trim() {
            "1" => {
                perform_operations_on_transformation_matrix(transformation_matrix);
            }

            "2" => {
                println!("Going back to initial operations.");
                initial_operations();
                // Add your initial operations logic here
            }

            "3" => {
                exit(0);
            }

            _ => {
                println!("Invalid choice. Please select 1, 2, or 3.");
            }
        }
    }
}

fn read_rotation_matrix() -> Option<Vec<f32>> {
    loop {
        let mut rotation_matrix_input = String::new();
        io::stdin()
            .read_line(&mut rotation_matrix_input)
            .expect("Failed to read rotation matrix");

        if rotation_matrix_input.trim().eq_ignore_ascii_case("Quit") {
            ordered_list_message(
                "Do you want to:\n1. Quit the program\n2. Go back to the list of options",
            );

            let mut user_choice = String::new();
            io::stdin()
                .read_line(&mut user_choice)
                .expect("Failed to read user choice");

            match user_choice.trim() {
                "1" => {
                    exit(0);
                }
                "2" => {
                    println!("Going back to list of options.");
                    initial_operations();
                    // Return None to indicate no rotation matrix.
                    return None;
                }
                _ => {
                    let error_message =
                        format!("Warning: Please enter either 1 or 2 for your choice.");
                    println!("{}", error_message.red());
                    continue;
                }
            }
        }

        let rotation_matrix_values: Vec<f32> = rotation_matrix_input
            .split(',')
            .map(|val| {
                val.trim().parse::<f32>().unwrap_or_else(|_| {
                    error_message("Invalid input for rotation matrix. Please enter a number.");
                    0.0 // You can choose a default value here if needed.
                })
            })
            .collect();

        if rotation_matrix_values.len() != 9 {
            let error_message =
                format!("Warning: Please enter exactly 9 values for the rotation matrix.");
            println!("{}", error_message.red());
            continue;
        }

        // If the input is valid, return Some(rotation_matrix_values).
        return Some(rotation_matrix_values);
    }
}

fn read_translation_vector() -> Vec<f32> {
    loop {
        let mut translation_vector_input = String::new();
        io::stdin()
            .read_line(&mut translation_vector_input)
            .expect("Failed to read translation vector");

        let translation_vector_values: Vec<f32> = translation_vector_input
            .split(',')
            .map(|val| {
                val.trim()
                    .parse::<f32>()
                    .unwrap_or_else(|_| {
                        error_message("Invalid input for translation vector. Please enter a number.");
                        0.0 // You can choose a default value here if needed.
                    })
            })
            .collect();

        if translation_vector_values.len() != 3 {
            let error_message =
                format!("Warning: Please enter exactly 3 values for the translation vector.");
            println!("{}", error_message.red());
            continue;
        }

        // If the input is valid, return the translation vector.
        return translation_vector_values;
    }
}

fn perform_operations_on_transformation_matrix(transformation_matrix: TransformationMatrix) {
    loop {
        ordered_list_message("What operation would you like to perform on the result?\n1. Inverse\n2. Multiply by another transformation matrix\n3. Multiply by an inverse transformation matrix\n4. Go back to initial operations \n5. Quit");

        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read user choice");

        match user_choice.trim() {
            "1" => {
                println!("Performing inverse operation on result.");
                println!("{}", transformation_matrix.inverse_transformation_matrix());
            }

            "2" => {
                println!("Operation 2");
                // Implement multiplication with another matrix
            }

            "3" => {
                println!("Operation 3");
                // Implement multiplication with inverse matrix
            }

            "4" => {
                println!("Done performing operations. Going back to initial operations.");
                initial_operations(); // Exit the loop
            }

            "5" =>{
                exit(0);
            }

            _ => {
                error_message("Invalid choice. Please select 1, 2, 3, or 4.");
            }
        }
    }
}

struct TransformationMatrix {
    rotation_matrix: Matrix3<f32>,
    translation_matrix: Matrix3x1<f32>,
}
impl TransformationMatrix {
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
