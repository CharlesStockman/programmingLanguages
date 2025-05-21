// Displays the result of the sum of two numbers
//
// Notes: 
//      * Use a function to sum the two numbers
//      * Use a funciton to display the result 
//      * Use the {:?} in the println! to display the result

fn add(num1:i32, num2:i32 ) -> i32 {
  return num1 + num2;  
}

fn display(num1:i32, num2:i32, result:i32) {
    println!(r#"{:?} + {:?} = {:?}"#, num1, num2, result)
}

fn main() {
    
    let a: i32 = 5;
    let b: i32 = 3;
    let c: i32 = add(a,b);

    display(a,b,c);

}