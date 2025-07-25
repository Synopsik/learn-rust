fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let _tup = (500, 6.4, 1);

    let (x, y, z) = _tup;
    
    println!("{x} {y} {z}");

    // you can also access elements individually
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("{five_hundred}, {six_point_four}, {one}")
}