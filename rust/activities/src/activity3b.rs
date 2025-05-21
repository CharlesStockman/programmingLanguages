// Topic Flow Control
//
// Requirement:
//      If < 5 then display "<5"
//      If > 5 then display ">5"
//      if = 5 then display "=5"
fn main() {

    let mut tst_value:i8 = 4;
    logic(tst_value);

    tst_value = 6;
    logic(tst_value);

    tst_value = 5;
    logic(tst_value);


}

fn logic(tst_value: i8) {
    if tst_value < 5 {
       println!("<5");
    } else if tst_value > 5 {
       println!(">5");
    } else {
       println!("=5");
    }
}