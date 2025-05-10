fn main() {

    /*
    Take aways
    we can create variables using "let" - similar to java script

    for a variable to be changeable it must be create with the tag "mut"

    we use const to create constants
    name of a constant is written with all caps
    const is alive till the program is alive

     */

    let mut x = 5;
    println!("the value of x is {}",x);
    x = 6;
    println!("the value of x is {}",x);
    const SECONDS : i8=60;
    println!("the value of SECONDS is {}",SECONDS)
}
