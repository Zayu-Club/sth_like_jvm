pub mod attribute;
pub mod constant;
pub mod field;
pub mod method;

use std::io::Read;

use crate::jvm::class::{attribute::*, constant::*, field::*, method::*};
use crate::utils::{bytecode_reader::BytecodeReader, code2name};

#[derive(Debug)]
pub struct Class {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool: Vec<Constant>,
    pub access_flags: u16,
    pub this_class: String,
    pub super_class: String,
    pub interfaces: Vec<u16>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}

impl Class {
    pub fn new<T>(file: T) -> Option<Class>
    where
        T: Read,
    {
        let mut reader = BytecodeReader::read_from_file(file);

        let magic = reader.u32();
        if magic != 0xCAFEBABE_u32 {
            println!("It is not a classfile.");
            return Option::None;
        }
        let minor_version = reader.u16();
        let major_version = reader.u16();

        let constant_pool_count = reader.u16();

        let mut constant_pool: Vec<Constant> = Vec::new();
        for _ in 1..constant_pool_count {
            constant_pool.push(Constant::new(&mut reader));
        }

        let access_flags = reader.u16();
        let this_class_index = reader.u16();
        let this_class: String = match &constant_pool[this_class_index as usize - 1] {
            Constant::Class(constant_class) => {
                Constant::read_utf8_data(&constant_pool, constant_class.name_index)
            }
            _ => panic!("read class: wrong this_class_index."),
        };
        let super_class_index = reader.u16();
        let super_class: String = match &constant_pool[super_class_index as usize - 1] {
            Constant::Class(constant_class) => {
                Constant::read_utf8_data(&constant_pool, constant_class.name_index)
            }
            _ => panic!("read class: wrong super_class_index."),
        };

        let interfaces_count = reader.u16();
        let mut interfaces: Vec<u16> = Vec::new();
        for _ in 0..interfaces_count {
            interfaces.push(reader.u16());
        }

        let fields_count = reader.u16();
        let mut fields: Vec<Field> = Vec::new();
        for _ in 0..fields_count {
            fields.push(Field::new(&mut reader, &constant_pool));
        }

        let methods_count = reader.u16();
        let mut methods: Vec<Method> = Vec::new();
        for _ in 0..methods_count {
            methods.push(Method::new(&mut reader, &constant_pool));
        }

        let attributes_count = reader.u16();
        let mut attributes: Vec<Attribute> = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(Attribute::new(&mut reader, &constant_pool));
        }

        return Some(Class {
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        });
    }

    pub fn show_info(&self) {
        println!(">>> Constant Pool:");
        for cpi in 0..self.constant_pool.len() {
            println!("{:>3}: {:?}", cpi + 1, self.constant_pool[cpi]);
        }

        println!("--------------------------------------------------");
        println!("--------------------------------------------------");
        println!("{:#?}", self);
        println!("--------------------------------------------------");
        println!("--------------------------------------------------");

        for mi in 0..self.methods.len() {
            println!(
                ">>> Code {}: {}",
                self.methods[mi].name, self.methods[mi].descriptor,
            );
            for ai in 0..self.methods[mi].attributes.len() {
                match &self.methods[mi].attributes[ai] {
                    Attribute::Code(attribute_code) => {
                        for ci in 0..attribute_code.code.len() {
                            println!(
                                "    <{0:>3}|0x{0:0>2X}> ==> {1}",
                                attribute_code.code[ci],
                                code2name(attribute_code.code[ci]),
                            );
                        }
                        for aci in 0..attribute_code.attributes.len() {
                            match &attribute_code.attributes[aci] {
                                Attribute::LocalVariableTable(attr) => {
                                    for acli in 0..attr.local_variable_table.len() {
                                        println!("    {:?}", attr.local_variable_table[acli]);
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            println!("########################################")
        }
    }
}
