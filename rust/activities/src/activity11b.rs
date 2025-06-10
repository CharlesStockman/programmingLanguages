// Memory -- Show an exmaple of ownership

struct Item {
    quantity: i32,
    id: i32
}

fn main() {

    // 1. item_first belongs to main
    let item_first = Item {
        quantity: 5,
        id: 1
    };

    // 2. item_first has ownership tranferred to print_quantity 
    // 3. When the print_quantity routine is completed the memory for item_first is deallocated.
    print_quantity(item_first);

    // 4. Since item_first was deallocated calling "printQuanity" routine will cause the error "value used here after move."
    // print_id(item_first);

}

fn print_quantity(in_item: Item) {
    println!("The quantity is {:?}", in_item.quantity);
}

//fn print_id(in_item: Item) { 
//    println!("The id is {:?}", in_item.id);
//}