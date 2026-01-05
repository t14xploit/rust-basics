/*fn main() {
    println!("Hello, world!");
}*/

//fn = function

//main = program start

//println! is a macro (note the !)




//All vars by default are immutable in Rust
/*
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x=1;
    println!("The value of x is : {}", x);
}
*/
//make it mutable: 
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    let x=1;
    println!("The value of x is: {}", x);
}