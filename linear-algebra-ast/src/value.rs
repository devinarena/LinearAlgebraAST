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
    pub fn new_identity(size: usize) -> Self {
        let mut entries = Vec::new();
        for i in 0..size {
            for j in 0..size {
                entries.push(((i == j) as u32) as f64);
            }
        }
        Matrix {
            data: entries,
            rows: size,
            cols: size
        }
    }

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

    pub fn ref_matrix(&mut self) {
        // convert to row echelon form (pivots need not be 1)
        let mut i = 0;
        let mut j = 0;
        while i < self.rows && j < self.cols {
            let mut max_row = i;
            for k in i + 1..self.rows {
                if self.data[k * self.cols + j].abs() > self.data[max_row * self.cols + j].abs() {
                    max_row = k;
                }
            }
            if self.data[max_row * self.cols + j].abs() < 0.0000001 {
                j += 1;
                continue;
            }
            for k in 0..self.cols {
                let temp = self.data[i * self.cols + k];
                self.data[i * self.cols + k] = self.data[max_row * self.cols + k];
                self.data[max_row * self.cols + k] = temp;
            }
            for k in 0..self.rows {
                if k != i {
                    let c = -self.data[k * self.cols + j] / self.data[i * self.cols + j];
                    for l in 0..self.cols {
                        if l == j {
                            self.data[k * self.cols + l] = 0.0;
                        } else {
                            self.data[k * self.cols + l] += c * self.data[i * self.cols + l];
                        }
                    }
                }
            }
            i += 1;
            j += 1;
        }
    }

    pub fn rref_matrix(&mut self) {
        // convert to reduced row echelon form (pivots are 1)
        self.ref_matrix();
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.data[i * self.cols + j].abs() > 0.0000001 {
                    let c = 1.0 / self.data[i * self.cols + j];
                    for k in 0..self.cols {
                        self.data[i * self.cols + k] *= c;
                    }
                    break;
                }
            }
        }
    }

    pub fn inverse(&mut self) -> bool {
        if self.rows != self.cols {
            return false;
        }
        let iden = Matrix::new_identity(self.rows);
        let mut new_data = Vec::new();
        for i in 0..self.rows {
            for j in 0..self.cols {
                new_data.push(self.data[i * self.cols + j]);
            }
            for j in 0..iden.cols {
                new_data.push(iden.data[i * iden.cols + j]);
            }
        }
        self.data = new_data;
        self.cols *= 2;
        self.rref_matrix();
        for i in 0..self.rows {
            for j in 0..self.cols {
                if self.data[i * self.cols + j].abs() < 0.001 {
                    self.data[i * self.cols + j] = 0.0;
                }
            }
        }
        let mut new_data = Vec::new();
        for i in 0..self.rows {
            for j in self.cols / 2..self.cols {
                new_data.push(self.data[i * self.cols + j]);
            }
        }
        self.data = new_data;
        self.cols /= 2;
        true
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
