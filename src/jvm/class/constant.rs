use crate::utils::bytecode_reader::BytecodeReader;

#[derive(Debug)]
pub enum Constant {
    Utf8(ConstantUtf8),                             // tag: 0x01  1
    Integer(ConstantInteger),                       // tag: 0x03  3
    Float(ConstantFloat),                           // tag: 0x04  4
    Long(ConstantLong),                             // tag: 0x05  5
    Double(ConstantDouble),                         // tag: 0x06  6
    Class(ConstantClass),                           // tag: 0x07  7
    String(ConstantString),                         // tag: 0x08  8
    Fieldref(ConstantFieldref),                     // tag: 0x09  9
    Methodref(ConstantMethodref),                   // tag: 0x0a 10
    InterfaceMethodref(ConstantInterfaceMethodref), // tag: 0x0b 11
    NameAndType(ConstantNameAndType),               // tag: 0x0c 12
    MethodHandle(ConstantMethodHandle),             // tag: 0x0f 15
    MethodType(ConstantMethodType),                 // tag: 0x10 16
    Dynamic(ConstantDynamic),                       // tag: 0x11 17
    InvokeDynamic(ConstantInvokeDynamic),           // tag: 0x12 18
    Module(ConstantModule),                         // tag: 0x13 19
    Package(ConstantPackage),                       // tag: 0x14 20
}

impl Constant {
    pub fn new(reader: &mut BytecodeReader) -> Constant {
        let tag = reader.u8();
        match tag {
            0x1_u8 => {
                let length = reader.u16();
                let bytes = reader.read_as_string(length as usize);
                return Constant::Utf8(ConstantUtf8 { bytes });
            }
            0x3_u8 => {
                let bytes = reader.u32() as i32;
                return Constant::Integer(ConstantInteger { bytes });
            }
            0x4_u8 => {
                let bytes = reader.u32() as f32;
                return Constant::Float(ConstantFloat { bytes });
            }
            0x5_u8 => {
                let bytes = i64::from_be_bytes(
                    reader
                        .read_as_vec(8)
                        .try_into()
                        .expect("Failed try bytes into i64."),
                );
                return Constant::Long(ConstantLong { bytes });
            }
            0x6_u8 => {
                let bytes = f64::from_be_bytes(
                    reader
                        .read_as_vec(8)
                        .try_into()
                        .expect("Failed try bytes into f64."),
                );
                return Constant::Double(ConstantDouble { bytes });
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
            0xf_u8 => {
                let reference_kind = reader.u8();
                let reference_index = reader.u16();
                return Constant::MethodHandle(ConstantMethodHandle {
                    reference_kind,
                    reference_index,
                });
            }
            0x10_u8 => {
                let descriptor_index = reader.u16();
                return Constant::MethodType(ConstantMethodType { descriptor_index });
            }
            0x11_u8 => {
                let bootstrap_method_attr_index = reader.u16();
                let name_and_type_index = reader.u16();
                return Constant::Dynamic(ConstantDynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                });
            }
            0x12_u8 => {
                let bootstrap_method_attr_index = reader.u16();
                let name_and_type_index = reader.u16();
                return Constant::InvokeDynamic(ConstantInvokeDynamic {
                    bootstrap_method_attr_index,
                    name_and_type_index,
                });
            }
            0x13_u8 => {
                let name_index = reader.u16();
                return Constant::Module(ConstantModule { name_index });
            }
            0x14_u8 => {
                let name_index = reader.u16();
                return Constant::Package(ConstantPackage { name_index });
            }
            _ => panic!("unsupported constant tag: {}", tag),
        }
    }

    pub fn read_utf8_data(constant_pool: &Vec<Constant>, constant_index: u16) -> String {
        match &constant_pool[constant_index as usize - 1] {
            Constant::Utf8(c) => String::from(&c.bytes),
            _ => panic!("read Utf8: wrong ConstantUtf8 index."),
        }
    }
}

#[derive(Debug)]
pub struct ConstantUtf8 {
    pub bytes: String,
}
#[derive(Debug)]
pub struct ConstantInteger {
    pub bytes: i32,
}
#[derive(Debug)]
pub struct ConstantFloat {
    pub bytes: f32,
}
#[derive(Debug)]
pub struct ConstantLong {
    pub bytes: i64,
}
#[derive(Debug)]
pub struct ConstantDouble {
    pub bytes: f64,
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

#[derive(Debug)]
pub struct ConstantMethodHandle {
    pub reference_kind: u8,
    pub reference_index: u16,
}
#[derive(Debug)]
pub struct ConstantMethodType {
    pub descriptor_index: u16,
}
#[derive(Debug)]
pub struct ConstantDynamic {
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}
#[derive(Debug)]
pub struct ConstantInvokeDynamic {
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}
#[derive(Debug)]
pub struct ConstantModule {
    pub name_index: u16,
}
#[derive(Debug)]
pub struct ConstantPackage {
    pub name_index: u16,
}
