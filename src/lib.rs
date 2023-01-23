use std::fs;

#[derive(Debug)]
pub struct Matrix{
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>
}

// TODO: Redo Error as a its own struct
#[derive(Debug)]
enum Error {
    Type(String),
    Message(String),
}

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
            panic!("{:?} {:?}",Error::Type("DimensionError".to_string()), Error::Message("Not a valid matrix!".to_string()))        
        }
    
        return Matrix { rows: data.len(), cols: data[0].len(), data: data };
    }

    pub fn from_str(s: &str)->Matrix{
        let content: Vec<&str> = s
                                .split(";")
                                .collect();
        let data: Vec<Vec<f64>> = Self::parse_data(content, ",");

        if !Self::valid_matrix_contents(&data) {
            panic!("{:?} {:?}",Error::Type("DimensionError".to_string()), Error::Message("Not a valid matrix!".to_string()))        
        }
    
        return Matrix { rows: data.len(), cols: data[0].len(), data: data };
    }

    pub fn copy(&self)->Matrix{
        let mut data = Vec::new();
        for row in &self.data{
            data.push(row.to_vec())
        }
        return Matrix{rows: self.rows, cols: self.cols, data: data};
    }

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
        if self.cols != self.rows {
            panic!("{:?} {:?}",Error::Type("DimensionError".to_string()), Error::Message("Not a valid matrix!".to_string()))        
        }
        for r in 0..self.rows  {
            self.data[r][r] = 1.0;
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