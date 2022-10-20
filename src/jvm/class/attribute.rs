use std::vec;

use crate::jvm::class::constant::*;
use crate::utils::bytecode_reader::BytecodeReader;

#[derive(Debug)]
pub enum Attribute {
    ConstantValue(AttributeConstantValue),
    Code(AttributeCode),
    LineNumberTable(AttributeLineNumberTable),
    SourceFile(AttributeSourceFile),
    LocalVariableTable(AttributeLocalVariableTable),
}
impl Attribute {
    pub fn new(reader: &mut BytecodeReader, constant_pool: &Vec<Constant>) -> Attribute {
        let attribute_name_index = reader.u16();
        let attribute_name: String = Constant::read_utf8_data(constant_pool, attribute_name_index);

        let _attribute_length = reader.u32();
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
                let mut exception_table: Vec<ExceptionInfo> = Vec::new();
                for _ in 0..exception_table_length {
                    let start_pc = reader.u16();
                    let end_pc = reader.u16();
                    let handler_pc = reader.u16();
                    let catch_type = reader.u16();
                    exception_table.push(ExceptionInfo {
                        start_pc,
                        end_pc,
                        handler_pc,
                        catch_type,
                    });
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
                let sourcefile = Constant::read_utf8_data(constant_pool, sourcefile_index);
                return Attribute::SourceFile(AttributeSourceFile { sourcefile });
            }
            "LocalVariableTable" => {
                let local_variable_table_length = reader.u16();
                let mut local_variable_table: Vec<LocalVariableInfo> = Vec::new();
                for _ in 0..local_variable_table_length {
                    let start_pc = reader.u16();
                    let length = reader.u16();
                    let name_index = reader.u16();
                    let name = Constant::read_utf8_data(constant_pool, name_index);
                    let descriptor_index = reader.u16();
                    let descriptor = Constant::read_utf8_data(constant_pool, descriptor_index);
                    let index = reader.u16();

                    let local_variable_info = LocalVariableInfo {
                        start_pc,
                        length,
                        name,
                        descriptor,
                        index,
                    };
                    local_variable_table.push(local_variable_info);
                }
                return Attribute::LocalVariableTable(AttributeLocalVariableTable {
                    local_variable_table,
                });
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
    pub exception_table: Vec<ExceptionInfo>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug)]
pub struct ExceptionInfo {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug)]
pub struct AttributeLineNumberTable {
    pub line_number_table: Vec<Vec<u16>>,
}

#[derive(Debug)]
pub struct AttributeSourceFile {
    pub sourcefile: String,
}
#[derive(Debug)]
pub struct AttributeLocalVariableTable {
    pub local_variable_table: Vec<LocalVariableInfo>,
}

#[derive(Debug)]
pub struct LocalVariableInfo {
    pub start_pc: u16,
    pub length: u16,
    pub name: String,
    pub descriptor: String,
    pub index: u16,
}
