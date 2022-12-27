use std::io;

fn main() {

    println!("Hello, world!");

    // Variables

    let x = 4; // implicit type assignment
    // x = 5; cannot assign twice immutable variable IMMUTABLE
    let mut y = 4;
    y = y + 5;
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

    //Data Types
    //Primitive Types
    //Scalar

    //let x: u32 = 2; //Un-Signed Integer 32 bit
    //Basics on signed vs unsigned
    // u8 0 <-> 2^8 -1 = 0 to 255
    // i8 -2^7 <-> 2^7 -1 = -128 to 127

    let floating_point: f32 = 10.9; //f64 is default

    println!("playing with Floating point: {}", floating_point);

    //bool true = 1; false = 0

    let letter: char = 'a' ;

    println!("playing with char type: {}",letter);

   //Learning Tuples:

    let tup:  (i32,bool,char) = (8,true,'x');
    let tup2: (i8,bool,char) = (32,true,'x');

    println!("Playing with Tuples: {},{}", tup.0,tup2.0);

    //learning arrays
    let arr:[i32;5] = [1,2,3,4,5];
    println!("Playing with array: {}", arr[4]);

    // crate is package under that we have module.
   /* let mut input = String::new();
    println!("Waiting for User Input");
    // read console input
    io::stdin().read_line(&mut input).expect("error while reading input");

    println!("Playing with run time inputs: {}", input); */

    //Arithmetic and Type Casting

    let x: u8 = 9; //0 to 255
    let y: i8 = 10; //-128 to 127
   // ^ no implementation for `u8 + i8`
    // let z = x + y;

    let z = x + y as u8;
    println!("Arithmetic on different types: {}",z);

    let mut input = String::new();
    println!("Waiting for User Input only Integer");
    // read console input
    io::stdin().read_line(&mut input).expect("error while reading input");

    let int_input: i64 = input.trim().parse().unwrap();

    println!("Playing with run time inputs: {}", int_input);

    // conditional operators
    /*
    and &&
    or  ||
    not !
    */
    let cond = 2 as f32 <= 3.0;

    println!("{}", cond);

    if z == 19 {
        println!("Condition Matched");
    }else{
        println!("Condition didn't match");
    }

}
