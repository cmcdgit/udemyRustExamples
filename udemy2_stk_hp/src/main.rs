// use std::mem;
#![allow(dead_code)]

mod sh;                        // use mod to import a module to the main or any class

const MEANING_OF_LIFE:u8 = 42;   // no fixed address
static mut Z: i32 = 123;

fn main()
{
    sh::stack_and_heap();
    // operators();
    // scope_and_shadowing();
}

/*
fn operators()
{
    //arithmetic
    let mut a = 2+3*4;
    println!("a = {}", a);
    a = a+1;
    a -= 2;  //a = a-2
             // -=  +=  *=  %=

    println!("{} / {} = {}, with remainder: {}", a, 3, (a/3), (a%3));

    let a_cubed = i32::pow(a, 3);
    println!("a cubed = {}", a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("b = {}", b);
    println!("b_cubed = {}", b_cubed);
    println!("b_to_pi = {}", b_to_pi);


    // bitwise
    let c = 1 | 2;  //  | OR  //  & AND  //  ^ XOR  //  ! NOR
                    // 01 OR 10 = 11 == 3_10   (binary to decimal)
    println!("1|2 = {}", c);

    // calculating 2 to the power of 10  <<  there is also a shift >>
    let two_to_10 = 1 << 10;
    println!("2^10 = {}",  two_to_10);

    println!("1 << 10 = {}, binary = {:b}", 1 << 10, 1 << 10);
    println!("1 << 9 = {}, binary = {:b}", 1 << 9, 1 << 9);
    println!("1 << 8 = {}, binary = {:b}", 1 << 8, 1 << 8);
    println!("1 << 7 = {}, binary = {:b}", 1 << 7, 1 << 7);
    println!("1 << 6 = {}, binary = {:b}", 1 << 6, 1 << 6);
    println!("1 << 5 = {}, binary = {:b}", 1 << 5, 1 << 5);
    println!("1 << 4 = {}, binary = {:b}", 1 << 4, 1 << 4);
    println!("1 << 3 = {}, binary = {:b}", 1 << 3, 1 << 3);
    println!("1 << 2 = {}, binary = {:b}", 1 << 2, 1 << 2);
    println!("1 << 1 = {}, binary = {:b}", 1 << 1, 1 << 1);
    println!("1 << 0 = {}, binary = {:b}", 1 << 0, 1 << 0);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0;   //should be true
    println!("pi_less_4 : {}", pi_less_4);

}
*/
/*
fn scope_and_shadowing()
{
    let a = 123;        // <<<
    // let a = 777;        //
    {                     //
        let a = 777;     // <<< example of shadowing
        let b = 456;
        println!("Inside b = {}", b);
        println!("Inside a = {}", a);
    }
    println!("a = {}", a);
    // println!("Outside b = {}", b)

    println!("MEANING_OF_LIFE : {}", MEANING_OF_LIFE);
    
    unsafe                             // acknowledge that you are aware of the risk of 
    {                                   // having a mutable global var
        println!("Z : {}", Z);
    }

    // STACK : short term memory storage structure - allocation
    
        
    // HEAP : longer term memory storage structure - allocation : let x = Box::new(5)
        // the assigns a pointer to the value - retrieve : println!("{}", *x); 
}
*/







// fn scope_and_shadowing() 
// {
// let a = 123;
// let a = 1234;

// }
