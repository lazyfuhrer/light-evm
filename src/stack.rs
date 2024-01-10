#[derive(Debug)]
pub struct Stack {
    pub stack: Vec<usize>,
    pub max_depth: usize,
}

#[derive(Debug)]
pub enum StackError {
    InvalidItem(usize),
    StackOverflow,
    StackUnderflow,
}

impl Stack {
    pub fn new(max_depth: usize) -> Self {
        Self {
            stack: Vec::new(),
            max_depth,
        }
    }

    pub fn push(&mut self, item: usize) -> Result<(), StackError> {
        if item > usize::MAX {
            return Err(StackError::InvalidItem(item));
        }

        if self.stack.len() >= self.max_depth {
            return Err(StackError::StackOverflow);
        }

        self.stack.push(item);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<usize, StackError> {
        match self.stack.pop() {
            Some(item) => Ok(item),
            None => Err(StackError::StackUnderflow),
        }
    }
}