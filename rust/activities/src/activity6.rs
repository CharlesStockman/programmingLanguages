// Count down to 5 to 1 when completee and print Done... at the end

fn main() {

    let mut counter = 5;
    while counter > 0 {
        println!("{:?}", counter);
        counter = counter - 1;
    }
    println!("Done...");
}