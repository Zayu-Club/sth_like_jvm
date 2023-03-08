pub mod bytecode_reader;

use std::collections::HashMap;
use std::io::{BufRead, Cursor, Read};

pub fn parse_manifest(file: &mut zip::read::ZipFile) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();

    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let cursor = Cursor::new(data);
    for raw_line in cursor.lines() {
        let line = raw_line.unwrap();
        let pair: Vec<&str> = line.split(":").collect();
        if pair.len() < 2 {
            continue;
        }
        let key = String::from(pair[0].trim());
        let value = String::from(pair[1].trim());
        map.insert(key, value);
    }

    map
}

pub fn code2name(code: u8) -> &'static str {
    match code {
        000_u8 => return "nop              :Do nothing",
        001_u8 => return "aconst_null      :Push null",
        004_u8 => return "iconst_<1>       :Push int constant",
        005_u8 => return "iconst_<2>       :Push int constant",
        007_u8 => return "iconst_<4>       :Push int constant",
        016_u8 => return "lload            :Load long from local variable",
        026_u8 => return "iload_<0>        :Load int from local variable",
        027_u8 => return "iload_<1>        :Load int from local variable",
        028_u8 => return "iload_<2>        :Load int from local variable",
        042_u8 => return "aload_<0>        :Load reference from local variable",
        060_u8 => return "istore_<1>       :Store int into local variable",
        087_u8 => return "pop              :Pop the top operand stack value",
        096_u8 => return "iadd             :Add int",
        172_u8 => return "ireturn          :Return int from method",
        177_u8 => return "return           :Return void from method",
        178_u8 => return "getstatic        :Get static field from class",
        183_u8 => return "invokespecial    :Invoke instance method; direct invocation of instance initialization methods and methods of the current class and its supertypes",
        184_u8 => return "invokestatic     :Invoke a class (static) method",
        _      => return "!!!!! UNKNOW !!!!!",
    }
}
