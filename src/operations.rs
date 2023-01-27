use std::ops;
use crate::Matrix;
use crate::Error;

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

impl ops::Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut res = self.copy();
        for i in 0..self.rows {
            for j in 0..self.cols  {
                res.data[i][j] *= rhs;
            }
        }
        return res;
    }
}

impl ops::Mul<Matrix> for f64 {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut res = rhs.copy();
        for i in 0..rhs.rows {
            for j in 0..rhs.cols  {
                res.data[i][j] *= self;
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
