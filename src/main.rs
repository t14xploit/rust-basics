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

//make it mutable: 
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    let x=1;
    println!("The value of x is: {}", x);
} 

//test 1
fn main() {
    let x: u32 = 5;
    println!("The value of x is: {}", x);
    x = "hello";
    println!("The value of x is: {}", x);
}
//error

//test 2
fn main() {
    let x: u32 = 5;
    println!("The value of x is: {}", x);
    x = 5;
    println!("The value of x is: {}", x);
}

//error
*/ 




//Constants


//Constants can only be constant, they cannot be set to a function call or any other value that may change at runtime.

/*const HUNDRED_THOUSAND: u32 = 100_000;


//Notice how in Rust, we can use the _ character to denote a space in number without it affecting the value itself. This is purely for readability.


// name a constant in all uppercase.



//shadowing

fn main(){
    let x = 6;
    let x = x + 1;
    println!("{}", x)
}



// exercise
let mut tryhackme: u32 = 9;


//Every binary file written in Rust needs a main file, and every main file needs a main function.



//function 
fn hello() -> u16{
    println!("hello!");
    6
}

//returns 6

//Our main function does not return anything, which is the way it should be.


//arguments

fn print_name(name: String){
    println!("{}", name);
}
*/

//tests:

//Question 1

fn hello(){
    8172192: u16;
}

//Question 2

fn return(){
    6;
}

//Question 3

fn test(name) {
    println!("{}", name);
}
test("bee");