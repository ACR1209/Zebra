use zebra::Matrix;

fn main() {
    //let m =  Matrix::from_str("2,2;4,4");
    //let m2 =  Matrix::from_str("1,1;1,1");
    let m3 = Matrix::from_str("5,3,7,1;2,4,9,5;3,6,4,2;1,5,6,7");
    //m.apply(|x| x / 2.0);
    println!("{}", 3.0 * m3);
}


