use crate::jvm::class::{attribute::*, constant::*};
use crate::utils::bytecode_reader::BytecodeReader;

#[derive(Debug)]
pub struct Field {
    pub access_flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<Attribute>,
}

impl Field {
    pub fn new(reader: &mut BytecodeReader, constant_pool: &Vec<Constant>) -> Field {
        let access_flags = reader.u16();

        let name_index = reader.u16();
        let name: String = match &constant_pool[name_index as usize - 1] {
            Constant::Utf8(c) => String::from(&c.bytes),
            _ => panic!("read field: wrong name_index."),
        };

        let descriptor_index = reader.u16();
        let descriptor: String = match &constant_pool[descriptor_index as usize - 1] {
            Constant::Utf8(c) => String::from(&c.bytes),
            _ => panic!("read field: wrong descriptor_index."),
        };
        let attributes_count = reader.u16();
        let mut attributes: Vec<Attribute> = Vec::new();
        for _ in 0..attributes_count {
            attributes.push(Attribute::new(reader, constant_pool));
        }

        Field {
            access_flags,
            name,
            descriptor,
            attributes,
        }
    }
}
