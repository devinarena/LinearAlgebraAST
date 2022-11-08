
pub enum ValueType {
    SCALAR,
    VECTOR
}

pub struct Matrix {
    pub data: Vec<f64>,
    pub rows: usize,
    pub cols: usize
}

pub union Data {
    pub scalar: f64,
    pub matrix: std::mem::ManuallyDrop<Matrix>
}

pub struct Value {
    pub vtype: ValueType,
    pub data: Data,
}

impl Value {
    pub fn new_scalar(scalar: f64) -> Self {
        Value {
            vtype: ValueType::SCALAR,
            data: Data { scalar },
        }
    }
}