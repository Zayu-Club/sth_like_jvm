mod flag;
mod jvm;
mod utils;

use clap::Parser;
use flag::Flag;
use jvm::{class::Class, run_time::*};
use std::collections::HashMap;
use std::io::Result;
use std::{fs, io::BufReader};
use utils::parse_manifest;

use crate::jvm::class_loader::ClassLoader;

fn main() -> Result<()> {
    println!("########## start ##########");
    let flag = Flag::parse();

    let class_loader = ClassLoader::new(flag.jar);
    let main_class_name = class_loader.manifest.get("Main-Class").unwrap();
    println!("Main-Class: {}", main_class_name);
    let main_class = class_loader
        .class_map
        .get(&main_class_name.replace(".", "/"))
        .unwrap();
    main_class.show_info();

    let thread = Thread::new();

    Ok(())
}
