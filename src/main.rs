mod jvm;
mod utils;

use jvm::class::Class;
use std::io::Result;
use utils::bytecode_reader::BytecodeReader;

fn main() -> Result<()> {
    let classfile_path = "classfiles/simple/Add.class";
    let reader = BytecodeReader::read_from_classfile(classfile_path)?;
    let class = Class::new(reader);

    Ok(())
}
