// #![allow(dead_code)]
// #![allow(unused_variables)]

//UNIONS are primarily used for interoperating with C or C++ or similar languages
//not convienient  

//ENUMS can be used for an IntOrFloat operations
//   but you loose the compatibility of working with an API in C or C++

union IntOrFloat   //union doesn't take up 64bits only 32bits - 
{                    // it can be either int or float - not easy to know what is stored
    i:i32,
    f:f32
}

fn process_value(iof: IntOrFloat)
{
    unsafe {
        match iof {
            IntOrFloat { i: 42 } => {
                println!("meaning of life value");
            }

            IntOrFloat { f } => {                // f == f:32
                println!("value = {}", f)
            }
        }
    }
}

fn main() {
    let mut iof = IntOrFloat { i:123};
    iof.i = 234;
    
    let value = unsafe{ iof.i };             //because it is a union and can be either an int or a float
    println!("iof.i = {}", value);                                                //rust requires the unsafe keyword to allow this to compile

    process_value(IntOrFloat{i:42});       //meaning of life value
    process_value(IntOrFloat{f:42.0});     //value = 42.0
    process_value(IntOrFloat{i:5})         //this case will be missed by int part of IntOrFloat  
}
