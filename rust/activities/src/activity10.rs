// Create an expression using a boolean for >100 or <=100
// Using a match provide a string describing whether it is greater than or less than 100

fn main() {

    let mut value: i32;

    value = 50;
    show_comparison(value);

    value = 125;
    show_comparison(value);
}

fn show_comparison(input: i32) {

    let gt_100 = input > 100;
    let output: String = match gt_100 {
        true => String::from("Greater than 100 "),
        false => String::from("less then or equal to 100")
    };

    println!("The result is {:?}", output);
}