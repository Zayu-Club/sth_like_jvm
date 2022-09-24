use crate::utils::bytecode_reader::BytecodeReader;

#[derive(Debug)]
pub enum Constant {
    Utf8(ConstantUtf8),                             // tag: 0x1
    Integer(ConstantInteger),                       // tag: 0x3
    Float(ConstantFloat),                           // tag: 0x4
    Long(ConstantLong),                             // tag: 0x5
    Double(ConstantDouble),                         // tag: 0x6
    Class(ConstantClass),                           // tag: 0x7
    String(ConstantString),                         // tag: 0x8
    Fieldref(ConstantFieldref),                     // tag: 0x9
    Methodref(ConstantMethodref),                   // tag: 0xa
    InterfaceMethodref(ConstantInterfaceMethodref), // tag: 0xb
    NameAndType(ConstantNameAndType),               // tag: 0xc
}

impl Constant {
    pub fn new(reader: &mut BytecodeReader) -> Constant {
        let tag = reader.u8();
        match tag {
            0x1_u8 => {
                let length = reader.u16();
                let bytes = reader.read_as_String(length as usize);
                return Constant::Utf8(ConstantUtf8 { bytes });
            }
            0x7_u8 => {
                let name_index = reader.u16();
                return Constant::Class(ConstantClass { name_index });
            }
            0x8_u8 => {
                let string_index = reader.u16();
                return Constant::String(ConstantString { string_index });
            }
            0x9_u8 => {
                let class_index = reader.u16();
                let name_and_type_index = reader.u16();
                return Constant::Fieldref(ConstantFieldref {
                    class_index,
                    name_and_type_index,
                });
            }
            0xa_u8 => {
                let class_index = reader.u16();
                let name_and_type_index = reader.u16();
                return Constant::Methodref(ConstantMethodref {
                    class_index,
                    name_and_type_index,
                });
            }
            0xb_u8 => {
                let class_index = reader.u16();
                let name_and_type_index = reader.u16();
                return Constant::InterfaceMethodref(ConstantInterfaceMethodref {
                    class_index,
                    name_and_type_index,
                });
            }
            0xc_u8 => {
                let name_index = reader.u16();
                let descriptor_index = reader.u16();
                return Constant::NameAndType(ConstantNameAndType {
                    name_index,
                    descriptor_index,
                });
            }
            _ => panic!("unsupported constant tag: {}", tag),
        }
    }
}

#[derive(Debug)]
pub struct ConstantUtf8 {
    pub bytes: String,
}
#[derive(Debug)]
pub struct ConstantInteger {
    pub bytes: u32,
}
#[derive(Debug)]
pub struct ConstantFloat {
    pub bytes: u32,
}
#[derive(Debug)]
pub struct ConstantLong {
    pub high_bytes: u32,
    pub low_bytes: u32,
}
#[derive(Debug)]
pub struct ConstantDouble {
    pub high_bytes: u32,
    pub low_bytes: u32,
}
#[derive(Debug)]
pub struct ConstantClass {
    pub name_index: u16,
}
#[derive(Debug)]
pub struct ConstantString {
    pub string_index: u16,
}
#[derive(Debug)]
pub struct ConstantFieldref {
    pub class_index: u16,
    pub name_and_type_index: u16,
}
#[derive(Debug)]
pub struct ConstantMethodref {
    pub class_index: u16,
    pub name_and_type_index: u16,
}
#[derive(Debug)]
pub struct ConstantInterfaceMethodref {
    pub class_index: u16,
    pub name_and_type_index: u16,
}
#[derive(Debug)]
pub struct ConstantNameAndType {
    pub name_index: u16,
    pub descriptor_index: u16,
}
