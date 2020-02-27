

// In RUST a Vector is a resizable Array
fn vectors()
{
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);

    println!("a = {:?}", a);
    
    a.push(44);

    println!("a = {:?}", a);

    // datatypes usize isize 
    // let idx:u32 = 0;                // needs to be unsigned as negative indexes are not allowed
    let idx:usize = 0;                 // this works just fine

    a[idx] = 321;

    println!("a[0] = {}", a[idx]);
    println!("a[1] = {}", a[1]);
    println!("a[2] = {}", a[2]);
    println!("a[3] = {}", a[3]);

    // to ensure against getting an index out of bounds error you can check against the length (or)....
    // use the GET function
    let my_idx = 3;
    // Option
    match a.get(my_idx)
    {
        Some(x) => println!("a[{}] = {}", my_idx, x),
        None => println!("error, no such element")
    }

    // Iterate the vector
    println!("\nIterate the vector:");
    for x in &a { println!("from Vector: {}", x) };

    // Add value to a vector
    println!("\nAdd value to a vector:");
    a.push(567);
    for x in &a { println!("from Vector: {}", x) };

    // Pop the last element
    println!("\nPop the last element:");
    let _last_elem = a.pop();
    for x in &a { println!("from Vector: {}", x) };

    println!("\nlast elem : {:?}, a : {:?}", _last_elem, a);

    // Print last element as it gets popped
    println!("\nPrint last element as it gets popped:");
    while let Some(x) = a.pop()                // what is being asked here is - while a.pop yields a value 
    {                                          // (a Some(x) use that value to print it out)
        println!("{}", x)
    }
}

fn main() {
    vectors();
}
