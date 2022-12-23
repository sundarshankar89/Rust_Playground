fn main() {

    println!("Hello, world!");

    // Variables

    let x = 4; // implicit type assignment
    // x = 5; cannot assign twice immutable variable IMMUTABLE
    let mut y = 4;
    y = 5;
    println!("x is immutable variable: {}",x);
    println!("y is mutable variables: {}", y);

    //Defining Scope
    {
        let x = x - 2;
        println!("x is scope variable: {}",x);
    }


    let x = x + y  ;
    println!("x is recreated variable: {}",x);

    // Type Change
    let x = "hello";
    println!("x is Type Change variable: {}",x);

    // Constant
    const SECONDS_IN_MINS: u32 = 60;

    //const SECONDS_IN_MINS: u32 = 100; `SECONDS_IN_MINS` must be defined only once in the value namespace of this block

    println!("seconds in minutes: {}",SECONDS_IN_MINS);

}
