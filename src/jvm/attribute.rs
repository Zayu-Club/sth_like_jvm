use std::vec;

use crate::jvm::{constant::*, field::*, method::*};
use crate::utils::bytecode_reader::BytecodeReader;

#[derive(Debug)]
pub enum Attribute {
    ConstantValue(AttributeConstantValue),
    Code(AttributeCode),
    LineNumberTable(AttributeLineNumberTable),
    SourceFile(AttributeSourceFile),
}
impl Attribute {
    pub fn new(reader: &mut BytecodeReader, constant_pool: &Vec<Constant>) -> Attribute {
        let attribute_name_index = reader.u16();
        let attribute_name: String = match &constant_pool[attribute_name_index as usize - 1] {
            Constant::Utf8(c) => String::from(&c.bytes),
            _ => panic!("read attribute: wrong attribute_name_index."),
        };
        let attribute_length = reader.u32();
        match attribute_name.as_str() {
            "ConstantValue" => {
                let constantvalue_index = reader.u16();
                return Attribute::ConstantValue(AttributeConstantValue {
                    constantvalue_index,
                });
            }
            "Code" => {
                let max_stack = reader.u16();
                let max_locals = reader.u16();
                let code_length = reader.u32();
                let code: Vec<u8> = reader.read_as_vec(code_length as usize);
                let exception_table_length = reader.u16();
                let mut exception_table: Vec<Vec<u16>> = Vec::new();
                for _ in 0..exception_table_length {
                    exception_table.push(vec![
                        reader.u16(),
                        reader.u16(),
                        reader.u16(),
                        reader.u16(),
                    ]);
                }
                let attributes_count = reader.u16();
                let mut attributes: Vec<Attribute> = Vec::new();
                for _ in 0..attributes_count {
                    attributes.push(Attribute::new(reader, constant_pool));
                }
                return Attribute::Code(AttributeCode {
                    max_stack,
                    max_locals,
                    code,
                    exception_table,
                    attributes,
                });
            }
            "LineNumberTable" => {
                let line_number_table_length = reader.u16();
                let mut line_number_table: Vec<Vec<u16>> = Vec::new();
                for _ in 0..line_number_table_length {
                    line_number_table.push(vec![reader.u16(), reader.u16()]);
                }
                return Attribute::LineNumberTable(AttributeLineNumberTable { line_number_table });
            }
            "SourceFile" => {
                let sourcefile_index = reader.u16();
                let sourcefile = match &constant_pool[sourcefile_index as usize - 1] {
                    Constant::Utf8(c) => String::from(&c.bytes),
                    _ => panic!("read attribute: wrong sourcefile_index."),
                };
                return Attribute::SourceFile(AttributeSourceFile { sourcefile });
            }
            _ => panic!(
                "read attribute: unsupported attribute name: {}.",
                attribute_name
            ),
        }
    }
}

#[derive(Debug)]
pub struct AttributeConstantValue {
    // pub constantvalue: Constant,
    pub constantvalue_index: u16,
}

#[derive(Debug)]
pub struct AttributeCode {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<Vec<u16>>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct AttributeLineNumberTable {
    pub line_number_table: Vec<Vec<u16>>,
}

#[derive(Debug)]
pub struct AttributeSourceFile {
    pub sourcefile: String,
}
