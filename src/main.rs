use zebra::Matrix;

fn main() {
    let mut m =  Matrix::from_str("2,2;3,3");
    m.identity();
    println!("{}", m)
}
