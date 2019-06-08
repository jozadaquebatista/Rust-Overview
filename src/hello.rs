// any program will start by this main function like in C
fn main() {

  // here I created a variable to store a person name, in rust a varible must start with "let" keyword
  // "mut" keyword indicates a mutable variable, by default all variables in rust are immutable
  let mut name = "Mister Jhon";
  // if we specify "mut" keyword we can change variable contents
  name = "Mister Batata";

  // macro function from Rust, that shows content on command line
  println!("Hello {}!", name); // results into: "Hello Mister Batata!"

}
