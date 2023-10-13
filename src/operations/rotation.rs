use crate::exit;
use clap::error;
use colored::Colorize;
use nalgebra::Vector3;
use nalgebra::{Matrix3, Point3};
use std::f64;
use std::io;

use crate::messages::messages::{error_message, ordered_list_message};

pub fn rotation() {
    loop {
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
                let rotated_point = rotate_point_about_x_axis();
                println!("Rotated point: {}", rotated_point);
            }
            "2" => {
                let rotated_point = rotate_point_about_y_axis();
                println!("Rotated point: {}", rotated_point);
            }
            "3" => {
                let rotated_point = rotate_point_about_z_axis();
                println!("Rotated point: {}", rotated_point);
            }
            "4" => {
                let rotated_vector = rotate_vector_about_x_axis();
                println!("Rotated vector: {}", rotated_vector);
            }
            "5" => {
                let rotated_vector = rotate_vector_about_y_axis();
                println!("Rotated vector: {}", rotated_vector);
            }
            "6" => {
                let rotated_vector = rotate_vector_about_z_axis();
                println!("Rotated vector: {}", rotated_vector);
            }
            "7" => {}
            "8" => {}
            "9" => {}
            "10" => {
                exit(0);
            }
            _ => {
                error_message("Invalid choice. Please enter a valid number between 1 and 10.");
                continue;
            }
        }
    }
}
/// ~ This function asks the user to enter the angle in degrees or radians and returns a String
fn degrees_or_radians() -> String {
    loop {
        ordered_list_message("Degrees or radians?\n1. Degrees\n2. Radians");
        let mut user_choice = String::new();
        io::stdin()
            .read_line(&mut user_choice)
            .expect("Failed to read user choice");

        match user_choice.trim() {
            "1" => {
                println!("Degrees");
                return "Degrees".to_string(); // Return the choice
            }
            "2" => {
                println!("Radians");
                return "Radians".to_string(); // Return the choice
            }
            _ => {
                error_message("Invalid choice. Please enter either 1 and 2.");
                continue;
            }
        }
    }
}
/// ~ This function asks the user to enter the angle and returns a f64
fn read_angle() -> f64 {
    let angle_type = degrees_or_radians(); //~ Get the angle type from degrees_or_radians()
    let mut angle: f64;

    loop {
        println!("Enter the rotation angle:");
        let mut angle_input = String::new();
        io::stdin()
            .read_line(&mut angle_input)
            .expect("Failed to read angle");

        match angle_input.trim().parse::<f64>() {
            Ok(parsed_angle) => {
                angle = parsed_angle;
                if angle_type == "Degrees" {
                    angle = angle.to_radians(); // Convert to radians if angle_type is Degrees
                }
                break; // Valid input, exit the loop
            }
            Err(_) => {
                error_message("Invalid angle. Please enter a valid number.");
            }
        }
    }

    angle
}
/// ~ This function asks the user to enter the point and returns a Point3<f64>
fn read_point() -> Point3<f64> {
    loop {
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
            error_message("Invalid input for point. Please enter a valid point.");
            continue;
        }

        let point = Point3::new(point_values[0], point_values[1], point_values[2]);
        return point;
    }
}

/// ~ This function asks the user to enter the vector and returns a Vector3<f64>
fn read_vector() -> Vector3<f64> {
    loop {
        println!("Enter the vector in the following format: x,y,z");
        let mut vector_input = String::new();
        io::stdin()
            .read_line(&mut vector_input)
            .expect("Failed to read vector");

        let vector_values: Vec<f64> = vector_input
            .split(',')
            .map(|val| val.trim().parse::<f64>().expect("Invalid input for vector"))
            .collect();

        if vector_values.len() != 3 {
            error_message("Invalid input for vector. Please enter a valid vector.");
            continue;
        }

        let vector = Vector3::new(vector_values[0], vector_values[1], vector_values[2]);
        return vector;
    }
}

/// ~ function for rotation matrix about x axis
fn rotation_about_x(angle: f64) -> Matrix3<f64> {
    Matrix3::new(
        1.0,
        0.0,
        0.0,
        0.0,
        angle.cos(),
        -angle.sin(),
        0.0,
        angle.sin(),
        angle.cos(),
    )
}
/// ~ function for rotation matrix about y axis
fn rotation_about_y(angle: f64) -> Matrix3<f64> {
    Matrix3::new(
        angle.cos(),
        0.0,
        -angle.sin(),
        0.0,
        1.0,
        0.0,
        angle.sin(),
        0.0,
        angle.cos(),
    )
}
/// ~ function for rotation matrix about z axis
fn rotation_about_z(angle: f64) -> Matrix3<f64> {
    Matrix3::new(
        angle.cos(),
        -angle.sin(),
        0.0,
        angle.sin(),
        angle.cos(),
        0.0,
        0.0,
        0.0,
        1.0,
    )
}
/// ~ function to rotate a point about x axis
fn rotate_point_about_x_axis() -> Point3<f64> {
    let point = read_point();
    let angle = read_angle();
    let rotated_point = rotation_about_x(angle) * point;
    rotated_point
}

/// ~ function to rotate a point about y axis
fn rotate_point_about_y_axis() -> Point3<f64> {
    let point = read_point();
    let angle = read_angle();
    let rotated_point = rotation_about_y(angle) * point;
    rotated_point
}
/// ~ function to rotate a point about z axis
fn rotate_point_about_z_axis() -> Point3<f64> {
    let point = read_point();
    let angle = read_angle();
    let rotated_point = rotation_about_z(angle) * point;
    rotated_point
}
/// ~ function to rotate a vector about x axis
fn rotate_vector_about_x_axis() -> Vector3<f64> {
    let vector = read_vector();
    let angle = read_angle();
    let rotated_vector = rotation_about_x(angle) * vector;
    rotated_vector
}
/// ~ function to rotate a vector about y axis
fn rotate_vector_about_y_axis() -> Vector3<f64> {
    let vector = read_vector();
    let angle = read_angle();
    let rotated_vector = rotation_about_y(angle) * vector;
    rotated_vector
}
/// ~ function to rotate a vector about z axis
fn rotate_vector_about_z_axis() -> Vector3<f64> {
    let vector = read_vector();
    let angle = read_angle();
    let rotated_vector = rotation_about_z(angle) * vector;
    rotated_vector
}

// fn operations_to_perform_on_point()