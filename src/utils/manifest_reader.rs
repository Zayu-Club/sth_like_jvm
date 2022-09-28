use std::collections::HashMap;
use std::io::BufRead;
use std::io::Cursor;
use std::io::Read;
use std::io::Result;

pub fn parse_manifest(mut file: zip::read::ZipFile) -> Result<HashMap<String, String>> {
    let mut map: HashMap<String, String> = HashMap::new();

    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    let cursor = Cursor::new(data);
    for raw_line in cursor.lines() {
        let line = raw_line?;
        let pair: Vec<&str> = line.split(":").collect();
        if pair.len() < 2 {
            continue;
        }
        let key = String::from(pair[0].trim());
        let value = String::from(pair[1].trim());
        map.insert(key, value);
    }

    Ok(map)
}
