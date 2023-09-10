
use colored::Colorize;

pub fn warning_message(error_message: &str, values: &[&dyn std::fmt::Display]) {
    let values_str: String = values
        .iter()
        .map(|val| val.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    let warning_message = format!("Warning: {} {}", error_message, values_str);
    println!("{}", warning_message.yellow());
}

pub fn success_message(success_message: &str) {
    println!("{}", format!("Success: {}", success_message).green());
}

pub fn error_message(error_message: &str) {
    println!("{}", format!("Error: {}", error_message).red());
}

pub fn information_message(information_message: &str) {
    println!("{}", format!("Information: {}", information_message).blue());
}

pub fn highlighted_message(highlighted_message: &str) {
    println!("{}", highlighted_message.purple());
}

pub fn ordered_list_message(ordered_list_message: &str) {
    println!("{}", format!("{}", ordered_list_message).italic().blue());
}