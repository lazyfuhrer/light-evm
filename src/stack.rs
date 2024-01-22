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
    InvalidIndex
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

    pub fn peek(&mut self, index: usize) -> Result<usize, StackError> {
        if self.stack.len() <= index {
            return Err(StackError::StackUnderflow);
        }
        Ok(self.stack[self.stack.len() - (index + 1)])
    }

    pub fn swap(&mut self, index: usize) -> Result<(), StackError> {
        if index == 0 {
            return Err(StackError::InvalidIndex);
        }

        let stack_len = self.stack.len();

        if stack_len <= index {
            return Err(StackError::StackUnderflow);
        }

        self.stack.swap(stack_len - 1, stack_len - index - 1);

        Ok(())
    }
}