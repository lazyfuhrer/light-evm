use bytes::{BufMut, Bytes, BytesMut};

#[derive(Debug)]
pub struct Memory {
    pub memory: Vec<usize>,
}

#[derive(Debug)]
pub enum MemoryError {
    InvalidMemoryAccess(usize, usize),
    InvalidMemoryValue(usize, usize),
    InvalidOffset(usize)
}

impl Memory {
    const ZERO_WORD: [usize; 32] = [0; 32];

    pub fn new() -> Self {
        Self { memory: Vec::new() }
    }

    pub fn store(&mut self, offset: usize, value: usize) -> Result<(), MemoryError> {
        if value >= u8::MAX.into() {
            return Err(MemoryError::InvalidMemoryValue(offset, value));
        }

        self.expand_if_needed(offset);
        self.memory[offset] = value;
        Ok(())
    }

    pub fn load(&mut self, offset: usize) -> usize {
        if offset > self.memory.len() {
            return 0;
        }

        self.memory[offset]
    }

    // Todo: Return [u8;8] instead of Bytes?
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

    pub fn active_words(&self) -> usize {
        match self.memory.len().checked_div(32) {
            Some(v) => v,
            None => todo!(),
        }
    }

    fn expand_if_needed(&mut self, offset: usize) {
        if offset < self.memory.len() {
            return;
        }
        let active_words_after = std::cmp::max(self.active_words(), (offset + 1).div_ceil(32));
        let additional_words = active_words_after.saturating_sub(self.active_words());

        for _ in 0..additional_words {
            self.memory.extend_from_slice(&Self::ZERO_WORD);
        }
        return;
    }
}