use std::mem;

fn arrays()
{
    let mut a:[i32;5] = [1, 2, 3, 4, 5];              //5 32-bit integers

    println!("a has {} elements, first is {}", 
    a.len(), a[0]);

    a[0] = 321;
    println!("a has {} elements, first is now {}",
    a.len(), a[0]);

    println!("{:?}", a);

    if a != [1, 2, 3, 4, 5]
    {
        println!("does not match");
    }
    
    if a == [321, 2, 3, 4, 5]
    {
        println!("match")
    }
    else 
    {
        println!("check input");
    }
                                    // Control size of your arrays
    // let b = [1; 10];                       //40 bytes
    // let b = [1u16; 10];                    //20 bytes
    let b:[u8;10] = [1; 10];               //10 bytes
    for i in 0..b.len()
    {
        println!("#{} : {}", i, b[i]);
    }

    println!("b took up {} bytes", mem::size_of_val(&b));


    let mtx:[[f32;3]; 2] =
    [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", mtx);
    println!("mtx took up {} bytes", mem::size_of_val(&mtx));

    for i in 0..mtx.len()
    {
        for j in 0..mtx[i].len()
        {
            if i == j
            {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }

}

fn main() {
    arrays();
}
