pub struct Memory {
    pub data: Vec<u8>, // your RAM stored as bytes
}

impl Memory {
    pub fn new() -> Self {
        // returns type Memory
        Self {
            data: vec![0; 4096],
        }
    }
}
