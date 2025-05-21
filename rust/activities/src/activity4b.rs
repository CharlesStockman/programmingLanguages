// Topic: Descision making with match
//
// Requirements:
//      1. if 1 then print("1")
//      2. if 2 then print("2")
//      3. if 3 then print("3")
//      4. anything else print other

fn main() {
    
    let mut bool_value:i8;

    bool_value = 1;
    logic(bool_value);

    bool_value = 2;
    logic(bool_value);

    bool_value = 3;
    logic(bool_value);

    bool_value = 4;
    logic(bool_value);

}

fn logic(_input_bool:i8) {

    match _input_bool {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("Its a number")
    }
}