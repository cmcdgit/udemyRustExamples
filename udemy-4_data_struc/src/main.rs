use std::mem;


// add comment
struct Point 
{
    x: f64,
    y: f64
}

// #[derive(Debug)]
// struct Line 
// {
//     start: Point,
//     end: Point
// }

fn structures()
{
    let p = Point { x: 3.0, y: 4.0 };
    println!("Point p is at ({}, {})", p.x, p.y);

    let p2 = Point { x: 5.0, y: 10.0 };
    println!("Point p2 is at ({}, {})", p2.x, p2.y);

    let my_line = Line { start: p, end: p2 };
    // println!("My line - Start: {}, End: {}", my_line.start, my_line.end);
}

fn main() {
    structures();
}
