// Using Struts and Enums print out the drinks flavor and ounces

enum Flavor {
    Pepsi
}

struct Drink {
    flavor: Flavor,
    ounces: f32
}

fn main() {

    let pepsi = Drink {
        flavor: Flavor::Pepsi,
        ounces: 20.0
    };

}