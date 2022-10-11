use crate::jvm::class::{attribute::*, constant::*, method::*, Class};

pub struct Thread {
    pub pc: usize,
    pub constant_pool: Vec<Constant>,
    pub stacks: Vec<Frame>,
    pub local_variables: Vec<Option<u32>>,
    pub operand_stacks: Vec<Option<u32>>,
}

impl Thread {
    pub fn new() -> Thread {
        todo!()
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
    pub pc: usize,
    pub code: Vec<u8>,
}

impl Frame {
    pub fn invoke_from_method_name(class: &Class, method_name: String) -> Frame {
        let mut method: &Method;
        for mi in 0..class.methods.len() {
            if method_name == class.methods[mi].name {
                method = &class.methods[mi];
                for ai in 0..method.attributes.len() {
                    match &method.attributes[ai] {
                        Attribute::Code(a) => {
                            return Frame {
                                pc: 0,
                                code: a.code.clone(),
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }
        panic!("unfound.")
    }

    fn read_code(&mut self) -> u8 {
        if self.pc >= self.code.len() {
            panic!("end of code, read code failed.")
        }
        self.pc += 1;
        self.code[self.pc - 1]
    }

    pub fn exec(&mut self) {
        while self.pc < self.code.len() {
            let code = self.read_code();
            match code {
                000_u8 => { /* do nothing */ }
                001_u8 => {
                    // push null
                    self.operand_stacks.push(Option::None);
                }
                002_u8 => {
                    // iconst_m1
                    let value = -1_i32 as u32;
                    self.operand_stacks.push(Some(value));
                }
                003_u8 => {
                    // iconst_0
                    let value = 0_i32 as u32;
                    self.operand_stacks.push(Some(value));
                }
                004_u8 => {
                    // iconst_1
                    let value = 1_i32 as u32;
                    self.operand_stacks.push(Some(value));
                }
                005_u8 => {
                    // iconst_2
                    let value = 2_i32 as u32;
                    self.operand_stacks.push(Some(value));
                }
                006_u8 => {
                    // iconst_3
                    let value = 3_i32 as u32;
                    self.operand_stacks.push(Some(value));
                }
                007_u8 => {
                    // iconst_4
                    let value = 4_i32 as u32;
                    self.operand_stacks.push(Some(value));
                }
                008_u8 => {
                    // iconst_5
                    let value = 5_i32 as u32;
                    self.operand_stacks.push(Some(value));
                }
                016_u8 => {
                    // bipush
                    let value: u32 = self.read_code() as i8 as u32;
                    self.operand_stacks.push(Some(value));
                }
                017_u8 => {
                    // sipush
                    let value: u32 = ((self.read_code() as u32) << 8) | self.read_code() as u32;
                    self.operand_stacks.push(Some(value));
                }
                018_u8 => {
                    // ldc
                    let const_pool_index = self.read_code() as usize;
                    match &self.constant_pool[const_pool_index - 1] {
                        Constant::Integer(i) => {
                            self.operand_stacks.push(Some(i.bytes as u32));
                        }
                        _ => {
                            panic!(
                                "unsupported constant, const_pool_index:{}, {:?}",
                                const_pool_index - 1,
                                self.constant_pool[const_pool_index - 1]
                            )
                        }
                    }
                }

                184_u8 => {
                    let static_method_index =
                        (((self.read_code() as u32) << 8) | self.read_code() as u32) as usize;
                    match &self.constant_pool[static_method_index - 1] {
                        Constant::Methodref(m) => {}
                        _ => {
                            panic!(
                                "unsupported static method, const_pool_index:{}, {:?}",
                                static_method_index - 1,
                                self.constant_pool[static_method_index - 1]
                            )
                        }
                    }
                }
                _ => panic!("unsupported code: {}", code),
            }
        }
    }
}
