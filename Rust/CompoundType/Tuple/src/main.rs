fn main() {

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let(x, y, z) = tup;

    println!("the value of y is:{}", y);

    //there other ways to access the tuple, like:

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{}", one)
}
