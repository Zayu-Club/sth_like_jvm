mod jvm;
mod utils;

use clap::{Parser, Subcommand};
use jvm::run_time::*;
use std::{collections::HashMap, fs, io::BufReader};

use crate::jvm::class_loader::ClassLoader;
use crate::utils::parse_manifest;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(long, default_value = ".")]
    classpath: String,
    // debug: bool,
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Jar {
        path: String,
        args: Vec<String>,
    },
    Run {
        class_name: String,
        args: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    println!("########## start ##########");

    let mut class_loader = ClassLoader::new();
    class_loader.load_from_path(cli.classpath);

    match &cli.command {
        Some(Commands::Jar { path, args }) => {
            // class_loader.load_from_path(path.to_string());
            let jar_file = fs::File::open(path).unwrap();
            let mut jar_archive = zip::ZipArchive::new(BufReader::new(jar_file)).unwrap();
            let manifest: HashMap<String, String> =
                parse_manifest(&mut jar_archive.by_name("META-INF/MANIFEST.MF").unwrap());
            let main_class_name = manifest.get("Main-Class").unwrap();
            println!("Main-Class: {}", main_class_name);

            class_loader
                .class_map
                .get(&main_class_name.replace(".", "/"))
                .unwrap()
                .show_info();
            let mut thread = Thread::new(class_loader.class_map);

            thread.invoke_from_method_name(String::from(main_class_name), String::from("main"));
            thread.run(args.to_vec());
        }
        Some(Commands::Run { class_name, args }) => {
            let mut thread = Thread::new(class_loader.class_map);

            thread.invoke_from_method_name(String::from(class_name), String::from("main"));
            thread.run(args.to_vec());
        }
        None => {}
    }
}
