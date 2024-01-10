use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug)]
pub struct Memory {
    pub memory: Vec<usize>,
}

#[derive(Debug)]
pub enum MemoryError {
    InvalidMemoryAccess(usize, usize),
    InvalidMemoryValue(usize, usize),
}

impl Memory {
    pub fn new() -> Self {
        Self { memory: Vec::new() }
    }

    pub fn store(&mut self, offset: usize, value: usize) {
        // Memory expansion
        if self.memory.len() <= offset {
            self.memory.resize(1, 0);
        }

        self.memory[offset] = value;
    }

    pub fn load(&mut self, offset: usize) -> usize {
        if offset > self.memory.len() {
            return 0;
        }

        self.memory[offset]
    }

    pub fn load_range(&mut self, offset: usize, length: usize) -> Bytes {
        let mut bytes = BytesMut::new();
        for i in offset..offset + length {
            let data = self.load(i).to_ne_bytes(); // Convert usize to [u8; 8]
            for byte in &data {
                bytes.put_u8(*byte);
            }
        }
        bytes.freeze() // Convert BytesMut to Bytes
    }
}