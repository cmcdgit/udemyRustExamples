fn main() {
    let x = 3.0;
    let y = 2.0;

    // Option -> Some(v) | None
    let result =
        if y != 0.0 { Some(x/y)} else { None };

    match result {
        Some(z) => println!("{}/{} = {}", x, y, z),
        None => println!("cannot divide by zero")
    }

    if let Some(z) = result {       // handler for checking whether the result is None or not
        println!("<if> result = {}", z)
    }

    while let Some(z) = result {      // checks if what's on the left can be assigned to what's on the right
        println!("<while> result = {}", z);
        break;                       // remove break for a continous loop
    }

}
