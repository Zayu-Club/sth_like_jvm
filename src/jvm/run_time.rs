use std::collections::HashMap;

use crate::jvm::class::{attribute::*, constant::*, method::*, Class};

pub struct Thread {
    pub pc: usize,
    pub class_map: HashMap<String, Class>,
    pub stacks: Vec<Frame>,
    pub local_variables: Vec<Option<u32>>,
    pub operand_stacks: Vec<Option<u32>>,
}

impl Thread {
    pub fn new(class_map: HashMap<String, Class>) -> Thread {
        return Thread {
            pc: 0,
            class_map,
            stacks: Vec::new(),
            local_variables: Vec::new(),
            operand_stacks: Vec::new(),
        };
    }

    pub fn invoke_from_method_name(&mut self, class_name: String, method_name: String) {
        println!(">>> Load {} - {}", class_name, method_name);
        let class_map = &self.class_map;
        let class = class_map.get(&class_name.replace(".", "/")).unwrap();

        for mi in 0..class.methods.len() {
            if method_name == class.methods[mi].name {
                let method = &class.methods[mi];
                for ai in 0..method.attributes.len() {
                    match &method.attributes[ai] {
                        Attribute::Code(a) => {
                            let f = Frame {
                                class_name: String::from(&class.this_class),
                                pc: 0,
                                code: a.code.clone(),
                            };
                            self.stacks.push(f);
                            return;
                        }
                        _ => continue,
                    }
                }
            }
        }
        panic!("unfound.")
    }

    pub fn pop_stacks(&mut self) -> Frame {
        match self.stacks.pop() {
            Some(it) => return it,
            None => panic!("jvm stack is empty"),
        };
    }

    pub fn run(&mut self) {
        println!(">>> BEGIN <<<");
        while self.stacks.len() != 0 {
            let top_frame = self.stacks.last_mut().unwrap();
            if top_frame.pc == top_frame.code.len() {
                self.pop_stacks();
            } else {
                self.exec();
            }
        }
        println!(">>>  END  <<<");
    }

    pub fn exec(&mut self) {
        let top_frame = self.stacks.last_mut().unwrap();
        let code = top_frame.read_code();
        println!(">>> [{code:0>3}]");
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
                let value: u32 = top_frame.read_code() as i8 as u32;
                self.operand_stacks.push(Some(value));
            }
            017_u8 => {
                // sipush
                let value: u32 =
                    ((top_frame.read_code() as u32) << 8) | top_frame.read_code() as u32;
                self.operand_stacks.push(Some(value));
            }
            018_u8 => {
                // ldc
                let const_pool_index = top_frame.read_code() as usize;
                let constant = &self
                    .class_map
                    .get(&top_frame.class_name)
                    .unwrap()
                    .constant_pool[const_pool_index - 1];
                match constant {
                    Constant::Integer(i) => {
                        self.operand_stacks.push(Some(i.bytes as u32));
                    }
                    _ => {
                        panic!(
                            "unsupported constant, const_pool_index:{}, {:?}",
                            const_pool_index - 1,
                            constant
                        )
                    }
                }
            }
            026_u8 => {}
            027_u8 => {}
            087_u8 => {}
            096_u8 => {}
            172_u8 => {}
            177_u8 => {}
            184_u8 => {
                let static_method_index =
                    (((top_frame.read_code() as u32) << 8) | top_frame.read_code() as u32) as usize;
                let constant = &self
                    .class_map
                    .get(&top_frame.class_name)
                    .unwrap()
                    .constant_pool[static_method_index - 1];
                match constant {
                    Constant::Methodref(m) => {
                        let mut class_name: String;
                        let mut name: String;
                        let mut descriptor: String;
                        match &self
                            .class_map
                            .get(&top_frame.class_name)
                            .unwrap()
                            .constant_pool[m.class_index as usize - 1]
                        {
                            Constant::Class(c) => {
                                match &self
                                    .class_map
                                    .get(&top_frame.class_name)
                                    .unwrap()
                                    .constant_pool[c.name_index as usize - 1]
                                {
                                    Constant::Utf8(u) => {
                                        class_name = String::from(&u.bytes);
                                    }
                                    _ => panic!("Wrong index."),
                                }
                            }
                            _ => panic!("Wrong index."),
                        }
                        match &self
                            .class_map
                            .get(&top_frame.class_name)
                            .unwrap()
                            .constant_pool[m.name_and_type_index as usize - 1]
                        {
                            Constant::NameAndType(nt) => {
                                match &self
                                    .class_map
                                    .get(&top_frame.class_name)
                                    .unwrap()
                                    .constant_pool[nt.name_index as usize - 1]
                                {
                                    Constant::Utf8(u) => {
                                        name = String::from(&u.bytes);
                                    }
                                    _ => panic!("Wrong index."),
                                }
                                match &self
                                    .class_map
                                    .get(&top_frame.class_name)
                                    .unwrap()
                                    .constant_pool
                                    [nt.descriptor_index as usize - 1]
                                {
                                    Constant::Utf8(u) => {
                                        descriptor = String::from(&u.bytes);
                                    }
                                    _ => panic!("Wrong index."),
                                }
                            }
                            _ => panic!("Wrong index."),
                        }
                        self.invoke_from_method_name(class_name, name);
                    }
                    _ => {
                        panic!(
                            "unsupported static method, const_pool_index:{}, {:?}",
                            static_method_index - 1,
                            constant
                        )
                    }
                }
            }
            _ => panic!("unsupported code: {}", code),
        }
    }
}

pub struct Frame {
    pub class_name: String,
    pub pc: usize,
    pub code: Vec<u8>,
}

impl Frame {
    fn read_code(&mut self) -> u8 {
        if self.pc >= self.code.len() {
            panic!("end of code, read code failed.")
        }
        self.pc += 1;
        self.code[self.pc - 1]
    }
}
