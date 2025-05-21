// Topic: Functions
//
// Program Requirements:
//      * Display the first and last name
//
// Notes:
//      * Use a function to display your first name
//      * Use a function to display your last name
//      * Use a function to display display message to the terminal

fn display_first_name() -> String {
    return "Charles".to_string();
}

fn display_last_name() -> String {
    return "Stockman".to_string();
}

fn main() {
    println!("{} {}", display_first_name(), display_last_name());
}