#[derive(Clone)]
pub enum ValueType {
    SCALAR(Scalar),
    MATRIX(Matrix),
}

#[derive(Clone)]
pub struct Matrix {
    pub data: Vec<f64>,
    pub rows: usize,
    pub cols: usize,
}
impl Matrix {
    pub fn scale(&mut self, scalar: f64) {
        for i in 0..self.data.len() {
            self.data[i] *= scalar;
        }
    }

    pub fn transpose(&mut self) {
        let mut new_data = Vec::new();
        for i in 0..self.cols {
            for j in 0..self.rows {
                new_data.push(self.data[j * self.cols + i]);
            }
        }
        self.data = new_data;
        let temp = self.rows;
        self.rows = self.cols;
        self.cols = temp;
    }
}

#[derive(Clone)]
pub struct Scalar {
    pub data: f64,
}

#[derive(Clone)]
pub struct Value {
    pub data: ValueType,
}

impl Value {
    pub fn new_scalar(scalar: f64) -> Self {
        Value {
            data: ValueType::SCALAR(Scalar { data: scalar }),
        }
    }

    pub fn new_matrix(matrix: Vec<f64>, rows: usize, cols: usize) -> Self {
        Value {
            data: ValueType::MATRIX(Matrix {
                data: matrix,
                rows,
                cols,
            }),
        }
    }

    pub fn wrap_matrix(matrix: Matrix) -> Self {
        Value {
            data: ValueType::MATRIX(matrix),
        }
    }

    pub fn print(&self) {
        match &self.data {
            ValueType::SCALAR(s) => println!("{}", s.data),
            ValueType::MATRIX(m) => {
                for i in 0..m.rows {
                    for j in 0..m.cols {
                        if j == 0 {
                            print!("| ");
                        }
                        print!("{} ", m.data[i * m.cols + j]);
                        if j == m.cols - 1 {
                            print!("|");
                        }
                    }
                    println!();
                }
            }
        }
    }
}
