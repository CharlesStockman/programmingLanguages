// Print whether the y value is less then 5, equal to five or greater than 5

fn main() {

    let tuple1 = (3,1);
    y_value_comparison_to_5(tuple1);

    let tuple2 = (3,5);
    y_value_comparison_to_5(tuple2);

    let tuple3 = (3, 7);
    y_value_comparison_to_5(tuple3);

}

fn y_value_comparison_to_5(coord: (i32, i32)) {
    let (x, y ) = coord;
    if coord.1 < 5 {
        println!("The y-coordinate: {:?} is less than 5", coord.1)
    } else if coord.1 == 5 {
        println!("The y-coordinate: {:?} is equal to 5", coord.1)
    } else {
        println!("The y-coordinate: {:?} is greater than 5", coord.1)
    }
}