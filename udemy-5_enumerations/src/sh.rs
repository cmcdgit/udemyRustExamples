#![allow(dead_code)]
#![allow(unused_variables)]

use std::mem;

    // STACK : short term memory storage structure - allocation
    
        
    // HEAP : longer term memory storage structure - allocation : let x = Box::new(5)
        // the assigns a pointer to the value - retrieve : println!("{}", *x); 

struct Point
{
    x: f64,
    y: f64
}

fn origin() -> Point
{
    Point{x: 1.0, y: 0.0}
}

pub fn stack_and_heap() 
{
    let p1 = origin();                               
    let p2 = Box::new(origin());

    println!("p1 on the stack takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 on the heap takes up {} bytes", mem::size_of_val(&p2));
    // p2 uses less memory because it is only a pointer to an address (64-bit system)

    let p3 = *p2;                         // * lets you follow where the boxed value is
    println!("p3.x : {}, p3.y : {}", p3.x, p3.y);                 // .x is the x coordinate from the Point
}