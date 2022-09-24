use crate::jvm::{attribute::*, constant::*, field::*, method::*};
use crate::utils::bytecode_reader::BytecodeReader;

#[derive(Debug)]
pub struct Class {
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    constant_pool: Vec<Constant>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<u16>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    attributes: Vec<Attribute>,
}

impl Class {
    pub fn new(mut reader: BytecodeReader) -> Option<Class> {
        let magic = reader.u32();
        if magic != 0xCAFEBABE_u32 {
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
        let this_class = reader.u16();
        let super_class = reader.u16();

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
            constant_pool_count,
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
}
