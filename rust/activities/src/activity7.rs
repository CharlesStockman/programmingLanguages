// Topics: Working with an enum
//
// Requirements print the color's name to the terminal

enum Color {
    Red,
    Green,
    Blue,
}

fn main() {

    let mut my_color;

    my_color = Color::Red;
    println!("{:?}", get_color_name(my_color));

    my_color = Color::Green;
    println!("{:?}", get_color_name(my_color));

    my_color = Color::Blue;
    println!("{:?}", get_color_name(my_color));
}

fn get_color_name(chosen_color: Color) -> String {
     match chosen_color {
        Color::Red => String::from("Red"),
        Color::Blue => String::from("Blue"),
        Color::Green => String::from("Green")
     }

}