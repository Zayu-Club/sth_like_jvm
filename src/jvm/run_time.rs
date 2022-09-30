use crate::jvm::class::{method::Method, Class};

pub struct Thread {
    pub pc: usize,
    pub stacks: Vec<Frame>,
}

impl Thread {
    pub fn new() -> Thread {
        return Thread {
            pc: 0,
            stacks: Vec::new(),
        };
    }
    pub fn pop_stacks(&mut self) -> Frame {
        match self.stacks.pop() {
            Some(it) => return it,
            None => panic!("jvm stack is empty"),
        };
    }
    pub fn push_stacks(&mut self, f: Frame) {
        self.stacks.insert(0, f);
    }
}

pub struct Frame {
    pub code: Vec<u32>,
    pub max_locals: usize,
    pub local_variables: Vec<u32>,
    pub max_stack: usize,
    pub operand_stacks: Vec<u32>,
}

impl Frame {
    pub fn new(max_stack: usize, max_locals: usize) -> Frame {
        return Frame {
            code: Vec::new(),
            max_locals,
            local_variables: Vec::new(),
            max_stack,
            operand_stacks: Vec::new(),
        };
    }

    fn push_local_variables(&mut self, value: u32) {
        if self.local_variables.len() + 1 > self.max_stack {
            panic!("push local_variables error.")
        }
        self.local_variables.insert(0, value);
    }

    fn push_operand_stacks(&mut self, value: u32) {
        if self.operand_stacks.len() + 1 > self.max_stack {
            panic!("push operand_stacks error.")
        }
        self.operand_stacks.insert(0, value);
    }

    fn pop_local_variables(&mut self) -> u32 {
        match self.local_variables.pop() {
            Some(it) => return it,
            None => panic!("pop local_variables error."),
        };
    }

    fn pop_operand_stacks(&mut self) -> u32 {
        match self.operand_stacks.pop() {
            Some(it) => return it,
            None => panic!("pop operand_stacks error."),
        };
    }

    pub fn exec(&self) {
        for ci in 0..self.code.len() {
            match self.code[ci] {
                _ => {}
            }
        }
    }
}
