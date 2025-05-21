// Topic: Decision using a match
//
// Requirements
//      Display "Its true" if the value is true
//      Display "Its false" if the value is false
//

fn main() {

    let mut bool_value:bool;

    bool_value = false;
    logic(bool_value);

    bool_value = true;
    logic(bool_value);
}

fn logic(_bool_value: bool) {
    
    match _bool_value {
        true => println!("Its true"),
        false => println!("its false")
    }

}