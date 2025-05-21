// Loopign using the loop statement
//
// Program requuirements
//  1. Display 1 through 4 in the terminal
//

fn main() {

    let mut index = 1;
    loop {
        println!("{:?}", index);
        if index == 4 {
            break;
        }
        index = index + 1;
    }
}