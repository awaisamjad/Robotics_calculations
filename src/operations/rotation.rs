use colored::Colorize;
use nalgebra::{Matrix3, Matrix4, Point3, Vector4};
use std::f64;
use std::io;
use crate::print_operations_and_take_input;

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