
#[derive(Clone)]
pub enum ValueType {
    SCALAR(Scalar),
    MATRIX(Matrix),
}

#[derive(Clone)]
pub struct Matrix {
    pub data: Vec<f64>,
    pub rows: usize,
    pub cols: usize
}

#[derive(Clone)]
pub struct Scalar {
    pub data: f64
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
            data: ValueType::MATRIX(Matrix { data: matrix, rows, cols }),
        }
    }
}