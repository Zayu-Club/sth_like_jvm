use crate::jvm::class::Class;
use std::{collections::HashMap, fs, io::BufReader};

pub struct ClassLoader {
    // pub manifest: HashMap<String, String>,
    pub class_map: HashMap<String, Class>,
}

impl ClassLoader {
    pub fn new() -> ClassLoader {
        // let jar_file = fs::File::open(file_path).unwrap();
        // let mut jar_archive = zip::ZipArchive::new(BufReader::new(jar_file)).unwrap();

        // let manifest: HashMap<String, String> =
        //     parse_manifest(&mut jar_archive.by_name("META-INF/MANIFEST.MF").unwrap());
        // let mut class_map: HashMap<String, Class> = HashMap::new();

        // for i in 0..jar_archive.len() {
        //     let ziped_file = jar_archive.by_index(i).unwrap();
        //     if ziped_file.is_file() {
        //         if ziped_file.name().ends_with(".class") {
        //             let class = Class::new(ziped_file).unwrap();
        //             let class_name = String::from(&class.this_class);

        //             class_map.insert(class_name, class);
        //         }
        //     }
        // }

        // let mut manifest: HashMap<String, String> = HashMap::new();
        let class_map: HashMap<String, Class> = HashMap::new();
        ClassLoader {
            // manifest,
            class_map,
        }
    }

    pub fn load_from_path(&mut self, class_path: String) {
        // BAD CODE
        for path in class_path.split(";") {
            for entry in fs::read_dir(path).unwrap() {
                let entry_path = fs::canonicalize(entry.unwrap().path()).unwrap();
                let entry_path_string = entry_path.as_path().display().to_string();

                if entry_path.is_dir() {
                    self.load_from_path(entry_path_string)
                } else if entry_path.is_file() || entry_path.is_relative() {
                    if entry_path_string.ends_with(".class") {
                        let class = Class::new(fs::File::open(entry_path_string).unwrap()).unwrap();
                        let class_name = String::from(&class.this_class);
                        self.class_map.insert(class_name, class);
                    } else if entry_path_string.ends_with(".jar") {
                        let jar_file = fs::File::open(entry_path_string).unwrap();
                        let mut jar_archive =
                            zip::ZipArchive::new(BufReader::new(jar_file)).unwrap();
                        for i in 0..jar_archive.len() {
                            let ziped_file = jar_archive.by_index(i).unwrap();
                            if ziped_file.is_file() {
                                if ziped_file.name().ends_with(".class") {
                                    let class = Class::new(ziped_file).unwrap();
                                    let class_name = String::from(&class.this_class);
                                    self.class_map.insert(class_name, class);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn find_class(&self, class_name: &String) -> &Class {
        self.class_map.get(class_name).unwrap()
    }
}
