
pub struct Mode{
    value : i32,
    key: i32
}

impl Mode {
    pub fn new() -> Self{
        Mode{
            value: 0,
            key: 0
        }
    }

    pub fn set(&mut self, key: i32, value: i32){
        self.key = key;
        self.value = value;
    }

    pub fn get_key(&self) -> &i32 {
        &self.key
    }

    pub fn get_value(&self) -> &i32 {
        &self.value
    }
}