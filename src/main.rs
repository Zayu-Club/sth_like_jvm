mod flag;
mod jvm;
mod utils;

use clap::Parser;
use flag::Flag;
use jvm::run_time::*;

use crate::jvm::class_loader::ClassLoader;

fn main() {
    println!("\n\n########## start ##########\n\n");
    let flag = Flag::parse();

    let class_loader = ClassLoader::new(flag.jar);
    let main_class_name = class_loader.manifest.get("Main-Class").unwrap();
    println!("Main-Class: {}", main_class_name);

    class_loader
        .class_map
        .get(&main_class_name.replace(".", "/"))
        .unwrap()
        .show_info();
    let mut thread = Thread::new(class_loader.class_map);

    thread.invoke_from_method_name(String::from(main_class_name), String::from("main"));
    thread.run();
}
