use crate::{jvm::class::Class, utils::parse_manifest};
use std::{collections::HashMap, fs, io::BufReader};

pub struct ClassLoader {
    pub manifest: HashMap<String, String>,
    pub class_map: HashMap<String, Class>,
}

impl ClassLoader {
    pub fn new(file_path: String) -> ClassLoader {
        let jar_file = fs::File::open(file_path).unwrap();
        let mut jar_archive = zip::ZipArchive::new(BufReader::new(jar_file)).unwrap();

        let manifest: HashMap<String, String> =
            parse_manifest(&mut jar_archive.by_name("META-INF/MANIFEST.MF").unwrap());
        let mut class_map: HashMap<String, Class> = HashMap::new();
        for i in 0..jar_archive.len() {
            let ziped_file = jar_archive.by_index(i).unwrap();
            if ziped_file.is_file() {
                if ziped_file.name().ends_with(".class") {
                    let class = Class::new(ziped_file).unwrap();
                    let class_name = String::from(&class.this_class);

                    class_map.insert(class_name, class);
                }
            }
        }

        ClassLoader {
            manifest,
            class_map,
        }
    }

    pub fn find_super_class(&self, class_name: &String) -> &Class {
        self.class_map.get(class_name).unwrap()
    }
}
