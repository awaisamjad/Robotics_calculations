use colored::Colorize;
/// ~ This function prints a warning message to the console in yellow.
pub fn warning_message(error_message: &str, values: &[&dyn std::fmt::Display]) {
    let values_str: String = values
        .iter()
        .map(|val| val.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    let warning_message = format!("Warning => {} {}", error_message, values_str);
    println!("{}", warning_message.yellow());
}
/// ~ This function prints a success message to the console in green.
pub fn success_message(success_message: &str) {
    println!("{}", format!("Success => {}", success_message).green());
}
/// ~ This function prints an error message to the console in red.
pub fn error_message(error_message: &str) {
    println!("{}", format!("Error => {}", error_message).red());
}
/// ~ This function prints an information message to the console in blue.
pub fn information_message(information_message: &str) {
    println!(
        "{}",
        format!("Information => {}", information_message).blue()
    );
}
/// ~ This function prints a highlighted message to the console in purple.
pub fn highlighted_message(highlighted_message: &str) {
    println!("{}", highlighted_message.purple());
}
/// ~ This function prints an ordered list as a message in blue iwth italic style.
pub fn ordered_list_message(ordered_list_message: &str) {
    println!("{}", format!("{}", ordered_list_message).italic().blue());
}
/// ~ This function prints an exit message to the console in red and bold.
pub fn exit_message(exit_message: &str) {
    println!("{}", exit_message.red().bold());
}
