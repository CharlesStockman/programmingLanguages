// Using Struts and Enums print out the drinks flavor and ounces

enum Flavor {
    Pepsi,
    Red,
    Coffee
}

struct Drink {
    flavor: Flavor,
    ounces: f32
}

fn main() {

    let diet_pepsi = Drink {
        flavor: Flavor::Pepsi,
        ounces: 20.0
    };

    let wine = Drink {
        flavor: Flavor::Red,
        ounces: 32.0
    };

    let coffee = Drink {
        flavor: Flavor::Coffee,
        ounces: 8.0
    };

    println!("The drink is dies pepsi: Description - {:?} ounces - {:?}", flavor(diet_pepsi.flavor), diet_pepsi.ounces);
    println!("The drink is wine: Description       - {:?} ounces    - {:?}", flavor(wine.flavor), wine.ounces);
    println!("The drink is coffee: Description     - {:?} ounces  -  {:?}", flavor(coffee.flavor), coffee.ounces);

    

}

fn flavor(flavor: Flavor) -> String {
    match flavor {
        Flavor::Pepsi => String::from("Carbonated Goodness"),
        Flavor::Red => String::from("Stress Revliever"),
        Flavor::Coffee => String::from("Morning Pick Me Up")
    }
}