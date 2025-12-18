pub const NAME_SIZE: usize = 156;
pub const SIZE: usize = 12;
pub const TYPEFLAG_SIZE: usize = 1;

pub const ENTRY_SIZE: usize = NAME_SIZE + SIZE + TYPEFLAG_SIZE;

#[derive(PartialEq, Eq, Debug)]
pub enum EntryType {
    File,
    Directory,
}

impl EntryType {
    pub fn new(byte: u8) -> EntryType {
        match byte {
            b'0' => EntryType::File,
            b'1' => EntryType::Directory,
            _ => panic!("Unknown entry type"),
        }
    }
    pub fn as_byte(&self) -> u8 {
        match *self {
            EntryType::File => b'0',
            EntryType::Directory => b'1',
        }
    }
}

pub struct Header {
    pub name: [u8; NAME_SIZE],
    pub size: [u8; SIZE],
    pub typeflag: [u8; TYPEFLAG_SIZE],
}

impl Header {
    pub fn new() -> Header {
        Header {
            name: [0; NAME_SIZE],
            size: [0; SIZE],
            typeflag: [0; TYPEFLAG_SIZE],
        }
    }

    pub fn from_values(name: String, size: usize, typeflag: EntryType) -> Header {
        let mut header = Header::new();
        header.set_name(name);
        header.set_size(size);
        header.set_typeflag(typeflag);
        return header;
    }

    pub fn get_name(&self) -> String {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(NAME_SIZE);
        String::from_utf8_lossy(&self.name[..len]).to_string()
    }

    pub fn get_size(&self) -> usize {
        let tmp = String::from_utf8_lossy(&self.size);
        let size_str = tmp.trim_end_matches('\0').trim();

        usize::from_str_radix(size_str, 8).expect("Invalid octal in size field")
    }

    pub fn set_name(&mut self, name: String) {
        let bytes = name.as_bytes();
        let len = bytes.len().min(NAME_SIZE);

        self.name[..len].copy_from_slice(&bytes[..len]);
    }

    pub fn set_size(&mut self, size: usize) {
        let s = format!("{:0>11o}\0", size);
        let bytes = s.as_bytes();
        self.size[..bytes.len()].copy_from_slice(bytes);
    }

    pub fn set_typeflag(&mut self, typeflag: EntryType) {
        self.typeflag = [typeflag.as_byte()];
    }

    pub fn to_bytes(&self) -> [u8; ENTRY_SIZE] {
        let mut bytes = [0; ENTRY_SIZE];
        bytes[..NAME_SIZE].copy_from_slice(&self.name);
        bytes[NAME_SIZE..NAME_SIZE + SIZE].copy_from_slice(&self.size);
        bytes[NAME_SIZE + SIZE..NAME_SIZE + SIZE + TYPEFLAG_SIZE].copy_from_slice(&self.typeflag);
        return bytes;
    }

    pub fn from_bytes(bytes: [u8; ENTRY_SIZE]) -> Header {
        Header {
            name: bytes[0..NAME_SIZE].try_into().unwrap(),
            size: bytes[NAME_SIZE..NAME_SIZE + SIZE].try_into().unwrap(),
            typeflag: bytes[NAME_SIZE + SIZE..NAME_SIZE + SIZE + TYPEFLAG_SIZE]
                .try_into()
                .unwrap(),
        }
    }
}