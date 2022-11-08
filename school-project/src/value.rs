
#[derive(Copy, Clone)]
pub union Data {
    pub int: i32,
}

#[derive(Copy, Clone)]
pub struct Value {
    pub data: Data,
}

impl Value {
    pub fn new_int(int: i32) -> Self {
        Value {
            data: Data { int },
        }
    }
}