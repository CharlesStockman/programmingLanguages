// Flow Control Using if-else
//
// Program Requirements:
//   * Displays a message baded a boolean variable
//   * When the varialbe is set to true display hello
//   * When the variable is set to false display googbye

fn main() {
    let mut my_boolean: bool = false;
    
    determine_string(my_boolean);
    
    my_boolean = true;
    determine_string(my_boolean);
}

fn determine_string(my_boolean: bool)  {

    if my_boolean == true {
        println!("Hello");
    } else {
        println!("Good Bye");
    }
        
}