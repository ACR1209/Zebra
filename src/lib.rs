use std::fs;
use std::ops;
mod custom_error;
use crate::custom_error::Error;

#[derive(Debug)]
pub struct Matrix{
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>
}


/*
    BIG TODO: MUST MAKE THE CALCULATIONS AND THE STRUCT BE IN JUST A Vec<f64>
    TO BE FASTER AND MORE EFICIENT
*/

impl Matrix {
    // Create a new Matrix by initializing all the values to 0.0
    pub fn new(rows: usize, cols: usize) -> Matrix{
        let data = vec![vec![0.0; cols]; rows];
        return Matrix{rows, cols, data};
    }

    // Reads Matrix data from a file an initializes it
    pub fn from_file(path: &str) -> Matrix{
        let content = fs::read_to_string(path).unwrap_or_else(|e| panic!("{e}"));
        let data: Vec<Vec<f64>> = Self::parse_data(content.lines(), " ");
        
        if !Self::valid_matrix_contents(&data) {
            panic!("{}",Error::new("Not a valid matrix.".to_string(), "DimensionError".to_string()))        
        }
    
        return Matrix { rows: data.len(), cols: data[0].len(), data: data };
    }

    // Gets a new Matrix from a string with the format 2,3,4;2,4,5 where ; 
    // represents a new row and , a new element
    pub fn from_str(s: &str)->Matrix{
        let content: Vec<&str> = s
                                .split(";")
                                .collect();
        let data: Vec<Vec<f64>> = Self::parse_data(content, ",");

        if !Self::valid_matrix_contents(&data) {
            panic!("{}",Error::new("Not a valid matrix.".to_string(), "DimensionError".to_string()))        
        }
    
        return Matrix { rows: data.len(), cols: data[0].len(), data: data };
    }

    // Returns a copy of the Matrix
    pub fn copy(&self)->Matrix{
        let mut data = Vec::new();
        for row in &self.data{
            data.push(row.to_vec())
        }
        return Matrix{rows: self.rows, cols: self.cols, data: data};
    }

    // Parses the data passed to a matrix by it's iterator
    fn parse_data<I, S>(iterable: I, separator: &str) -> Vec<Vec<f64>>
        where I: IntoIterator<Item = S>,
            S: AsRef<str>
    {
        let mut data = Vec::new();

        for row in iterable {
            let mut data_row = Vec::new();
            let values: Vec<&str> = row.as_ref().split(separator).collect();

            for value in values {
                data_row.push(value.parse::<f64>().expect("Failed to parse value"));
            }

            data.push(data_row);
        }

        data
    }

    // Checks that all rows have the same amount of columns
    fn valid_matrix_contents(data: &Vec<Vec<f64>>) -> bool {
        let column_size = data[0].len();
        for row in data.iter() {
            if row.len() != column_size {
                return false;
            }
        }
        true
    }

    // Makes the current matrix the identity matrix
    pub fn identity(&mut self){
        if !self.is_square() {
            panic!("{}",Error::new("Not a squared matrix.".to_string(), "DimensionError".to_string()));      
        }
        for r in 0..self.rows  {
            self.data[r][r] = 1.0;
        }
    }

    fn is_square(&self)->bool{
        return self.cols == self.rows;
    }


    // Returns the biggest dimension of the matrix
    pub fn size(&self)->usize{
        return std::cmp::max(self.cols, self.rows);
    }

    // Applies a function to all elements of the Matrix
    pub fn apply(&mut self, f: impl Fn(f64)->f64){
        self.data = self.data.iter()
                             .map(|v|{
                                v.iter()
                                 .map(|x| f(*x))
                                 .collect()
                             })
                             .collect()
    }

    // Calculates the dot product or matrix multiplication 
    pub fn dot(&self, other: Matrix)->Matrix{
        if self.rows != other.cols || self.cols != other.rows{
            panic!("{}",  
                    Error::new(format!("Dimensions not matched. M1 is {}x{} and M2 is {}x{}", self.rows, self.cols, other.rows, other.cols)
                    .to_string(), 
                    "DimensionError"
                    .to_string()));
        }

        let mut dp = Matrix::new(self.rows, other.cols);
        for i in 0..dp.rows {
            for j in 0..dp.cols {
                let mut sum = 0.0;
                for k in 0..other.rows {
                    sum += self.data[i][k] * other.data[k][j];
                }

                dp.data[i][j] = sum
            }
        }   
        return dp;
    }

    pub fn rref(&mut self){
        if self.data[0][0] == 0.0{
            self.swap_rows(0);
        }

        let mut lead: usize = 0;
        let rows = self.rows;

        while lead < rows {
            for r in 0..rows  {
                let div = self.data[lead][lead];
                let mult = self.data[r][lead] / div;
                
                if r == lead {
                    self.data[lead] = self.data[lead]
                                          .iter()
                                          .map(|entry| entry / div)
                                          .collect();
                }else {
                    for c in 0..self.cols{
                        self.data[r][c] -= self.data [lead][c] * mult;
                    }
                }
            }
            lead+=1

        }
        self.correct();
    }

    pub fn det(&self)->f64{
        if !self.is_square() {
            panic!("{}",Error::new("Not a squared matrix.".to_string(), "DimensionError".to_string())); 
        }
        
        if self.rows == 2 && self.cols == 2{
            return self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0];
        }
        
        let row: usize = 1;
        let mut det = 0.0;

        for j in 0..self.cols {
            det += self.cofactor(row, j) * self.data[row][j];
        }

        return  det;
    }

    pub fn cofactor(&self, expanded_row: usize, j: usize) -> f64{
        let mut cut: Vec<Vec<f64>>= Vec::new();
        for r in 0..self.rows {
            if r == expanded_row {
                continue;
            }

            let mut v: Vec<f64> = Vec::new();
            for c in 0..self.cols {
                if c == j {
                    continue;
                }

                v.push(self.data[r][c]);
            }

            cut.push(v);
        }

        let n_r = cut.len();
        let n_c = cut[0].len();
        let minor = Matrix { rows: n_r, cols: n_c, data: cut}.det();
        let base: i32 = -1;
        return minor * f64::from(base.pow((expanded_row + j) as u32));
    }

    fn swap_rows(&mut self, row:usize){
        let mut n_r = 0;
        for r in 0..self.rows{
            if self.data[r][0] > 0.0{
                n_r = r;
                break;
            }
        }

        let temp = self.data[row].clone();
        self.data[row] = self.data[n_r].clone();
        self.data[n_r] = temp;
    }

    fn correct(&mut self){
        for row in 0..self.rows {
            for col in 0..self.cols {
                let val = self.data[row][col];
              
                if (val > 0.0 && val < 0.000001) || (val == -0.0) || (val < 0.0 && val > -0.000001){
                    self.data[row][col] = 0.0;
                }
                let floored = val.floor();

                if val - floored > 0.9999999{
                    self.data[row][col] = 1.0;
                }
                

            }
        }
    }

}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let whitespace = String::from_utf8(vec![b' '; self.cols * 2]).unwrap();
        write!(f, "┌ {}┐\n", whitespace)?;
        for val in &self.data {
            write!(f, "| ")?;
            for i in val {
                write!(f, "{} ", i)?;
            }
            write!(f, "|\n")?;
        }
        write!(f, "└ {}┘", whitespace)
    }
}

impl ops::Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Self::Output {
        if self.cols != rhs.cols || self.rows != rhs.rows{
            panic!("{}", 
            Error::new("Can't add two matrices of different dimensions"
                        .to_string(), 
                        "DimensionError"
                        .to_string()));
        }    
        let mut res = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows{
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + rhs.data[i][j];
            }
        }
        
        return res;
    }
}

impl ops::Sub<Matrix> for Matrix{
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Self::Output {
        if self.cols != rhs.cols || self.rows != rhs.rows{
            panic!("{}", 
            Error::new("Can't add two matrices of different dimensions"
                        .to_string(), 
                        "DimensionError"
                        .to_string()));
        }    
        let mut res = Matrix::new(self.cols, self.rows);
        for i in 0..self.rows{
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - rhs.data[i][j];
            }
        }
        
        return res;
    }
}

impl ops::Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        if self.rows != rhs.cols || self.cols != rhs.rows{
            panic!("{}",  
                    Error::new(format!("Dimensions not matched. M1 is {}x{} and M2 is {}x{}", self.rows, self.cols, rhs.rows, rhs.cols)
                    .to_string(), 
                    "DimensionError"
                    .to_string()));
        }

        let mut dp = Matrix::new(self.rows, rhs.cols);
        for i in 0..dp.rows {
            for j in 0..dp.cols {
                let mut sum = 0.0;
                for k in 0..rhs.rows {
                    sum += self.data[i][k] * rhs.data[k][j];
                }

                dp.data[i][j] = sum
            }
        }   
        return dp;
    }
}
