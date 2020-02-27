
fn use_slice(slice: &mut[i32])    // borrowing a slice of i32 data
{
    println!("first elem = {}, len = {}", slice[0], slice.len());
    slice[0] = 4321;
}

fn slices()
{
    let mut data = [1, 2, 3, 4, 5];

    // use_slice(&mut data);                     //full slice so index[0] of 'data' is changed to 4321
    use_slice(&mut data[1..4]);           //partial slice so index[1] of 'data' is changed to 4321

    println!("{:?}", data);
}


fn main() {
    slices();
}
