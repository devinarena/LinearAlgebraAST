
#[derive(Copy, Clone)]
pub union Data {
    pub scalar: i32,
}

#[derive(Copy, Clone)]
pub struct Value {
    pub data: Data,
}

impl Value {
    pub fn new_scalar(scalar: i32) -> Self {
        Value {
            data: Data { scalar },
        }
    }
}