mod flag;
mod jvm;
mod utils;

use clap::Parser;
use flag::Flag;
use jvm::{class::attribute::*, class::class::*, run_time::*};
use std::collections::HashSet;
use std::fs;
use std::io::BufReader;
use std::io::Result;
use utils::{bytecode_reader::*, manifest_reader::*};

use crate::jvm::class::constant::Constant;

fn main() -> Result<()> {
    println!("\n\n\n>>>> start >>>>\n\n\n");
    let flag = Flag::parse();

    // let jar_file = fs::File::open(&flag.jar).unwrap();
    // let mut jar_archive = zip::ZipArchive::new(BufReader::new(jar_file)).unwrap();
    // let mut MANIFEST_file = jar_archive.by_name("META-INF/MANIFEST.MF")?;
    // let manifest = parse_manifest(MANIFEST_file)?;
    // let main_class = manifest.get("Main-Class").unwrap();

    // for i in 0..jar_archive.len() {
    //     let ziped_file = jar_archive.by_index(i)?;
    //     if ziped_file.is_file() {
    //         println!("{:#?}", ziped_file.name());
    //     }
    // }
    // println!(">>>>>>>>>>>>>>>");
    // let reader = BytecodeReader::read_from_file(jar_archive.by_name("org/example/Add.class")?)?;

    let reader = BytecodeReader::read_from_file(fs::File::open(&flag.jar).unwrap())?;

    let class = Class::new(reader).unwrap();

    println!(">>>> Constant Pool >>>>");
    for cpi in 0..class.constant_pool.len() {
        println!("{:>2}: {:?}", cpi + 1, class.constant_pool[cpi]);
    }

    println!(">>>> Code Attribute in Methods >>>>");
    for mi in 0..class.methods.len() {
        for ai in 0..class.methods[mi].attributes.len() {
            match &class.methods[mi].attributes[ai] {
                Attribute::Code(attribute_code) => {
                    println!(
                        "{}: {}",
                        class.methods[mi].descriptor, class.methods[mi].name,
                    );
                    for ci in 0..attribute_code.code.len() {
                        println!(
                            "Code<{0:0>3}>:0x{0:0>2X} ==> {1}",
                            attribute_code.code[ci],
                            code2name(attribute_code.code[ci]),
                        );
                    }
                    println!("");
                }
                _ => {}
            }
        }
    }

    let thread = Thread::new();

    Ok(())
}

fn code2name(code: u8) -> &'static str {
    match code {
        184_u8 => return "invokestatic     :Invoke a class (static) method",
        060_u8 => return "istore_<1>       :Store int into local variable",
        005_u8 => return "iconst_<2>       :Push int constant",
        177_u8 => return "return           :Return void from method",
        004_u8 => return "iconst_<1>       :Push int constant",
        001_u8 => return "aconst_null      :Push null",
        183_u8 => return "invokespecial    :Invoke instance method; direct invocation of instance initialization methods and methods of the current class and its supertypes",
        026_u8 => return "iload_<0>        :Load int from local variable",
        000_u8 => return "nop              :Do nothing",
        007_u8 => return "iconst_<4>       :Push int constant",
        042_u8 => return "aload_<0>        :Load reference from local variable",
        027_u8 => return "iload_<1>        :Load int from local variable",
        096_u8 => return "iadd             :Add int",
        172_u8 => return "ireturn          :Return int from method",
        _      => return "!!!!! UNKNOW !!!!!",
    }
}
