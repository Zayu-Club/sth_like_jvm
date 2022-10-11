use std::io::{BufReader, Read};

pub struct BytecodeReader {
    pub offset: usize,
    pub size: usize,
    pub data: Vec<u8>,
}

impl BytecodeReader {
    pub fn new(raw_data: Vec<u8>) -> BytecodeReader {
        BytecodeReader {
            offset: 0,
            size: raw_data.len(),
            data: raw_data,
        }
    }
    pub fn read_from_file<T>(file: T) -> BytecodeReader
    where
        T: Read,
    {
        let mut buffer: Vec<u8> = Vec::new();
        BufReader::new(file).read_to_end(&mut buffer);
        BytecodeReader::new(buffer)
    }

    pub fn reset(&mut self) {
        self.offset = 0;
    }

    pub fn read_as_vec(&mut self, step: usize) -> Vec<u8> {
        let mut end: usize = self.offset + step;
        if end > self.size {
            panic!("BytecodeReader: incorrect length.")
        }
        let v = self.data[self.offset..end].to_vec();
        self.offset = end;
        v
    }

    pub fn read_as_string(&mut self, step: usize) -> String {
        let utf8 = self.read_as_vec(step);
        let s = String::from_utf8(utf8).unwrap();
        s
    }

    pub fn read_as_u64(&mut self, step: usize) -> u64 {
        if step < 1 || step > 8 {
            return 0_u64;
        }
        let mut result = self.read_as_vec(step);
        result.reverse();
        result.resize(8, 0u8);
        u64::from_le_bytes(
            result
                .as_slice()
                .try_into()
                .expect("Failed try result into u64."),
        )
    }

    pub fn u8(&mut self) -> u8 {
        self.read_as_u64(1) as u8
    }
    pub fn u16(&mut self) -> u16 {
        self.read_as_u64(2) as u16
    }
    pub fn u32(&mut self) -> u32 {
        self.read_as_u64(4) as u32
    }

    pub fn info(&mut self) {
        let t = self.offset;
        self.reset();
        let mut i = 1;
        while i <= self.size {
            print!("{:0>2X} ", self.u8());
            if i % 16 == 0 {
                print!("\n");
            }
            i += 1;
        }
        print!("\n");
        self.offset = t;
    }
}
