use std::{
    convert::TryInto,
    fs::File,
    io::{Cursor, Read, Write},
};

use crate::{
    gb::GameBoy,
    gen::{NAME, VERSION},
    util::capitalize,
};

pub trait Serialize {
    fn save(&self, buffer: &mut Vec<u8>);
    fn load(&mut self, data: &mut Cursor<Vec<u8>>);
}

pub struct BeesState {
    name: BeesName,
    info: BeesInfo,
    core: BeesCore,
}

impl Serialize for BeesState {
    fn save(&self, buffer: &mut Vec<u8>) {
        self.name.save(buffer);
        self.info.save(buffer);
        self.core.save(buffer);
    }

    fn load(&mut self, data: &mut Cursor<Vec<u8>>) {
        todo!()
    }
}

pub struct BeesBlockHeader {
    magic: String,
    size: u32,
}

impl BeesBlockHeader {
    pub fn new(magic: String, size: u32) -> Self {
        Self { magic, size }
    }
}

impl Serialize for BeesBlockHeader {
    fn save(&self, buffer: &mut Vec<u8>) {
        buffer.write_all(&self.magic.as_bytes()).unwrap();
        buffer.write_all(&self.size.to_le_bytes()).unwrap();
    }

    fn load(&mut self, data: &mut Cursor<Vec<u8>>) {
        let mut buffer = [0x00; 4];
        data.read_exact(&mut buffer).unwrap();
        self.magic = String::from_utf8(Vec::from(buffer)).unwrap();
        data.read_exact(&mut buffer).unwrap();
        self.size = u32::from_le_bytes(buffer.try_into().unwrap());
    }
}

pub struct BeesBuffer {
    size: u32,
    offset: u32,
}

pub struct BeesFooter {
    start_offset: u32,
    magic: u32,
}

pub struct BeesName {
    header: BeesBlockHeader,
    name: String,
}

impl BeesName {
    pub fn new(name: String) -> Self {
        Self {
            header: BeesBlockHeader::new(String::from("NAME"), name.len() as u32),
            name,
        }
    }
}

impl Serialize for BeesName {
    fn save(&self, buffer: &mut Vec<u8>) {
        self.header.save(buffer);
        buffer.write_all(self.name.as_bytes()).unwrap();
    }

    fn load(&mut self, data: &mut Cursor<Vec<u8>>) {
        let mut buffer = Vec::with_capacity(self.header.size as usize);
        buffer.resize(self.header.size as usize, 0);
        data.read_exact(&mut buffer).unwrap();
        self.name = String::from_utf8(Vec::from(buffer)).unwrap();
    }
}

pub struct BeesInfo {
    header: BeesBlockHeader,
    title: [u8; 16],
    checksum: [u8; 2],
}

impl Serialize for BeesInfo {
    fn save(&self, buffer: &mut Vec<u8>) {}

    fn load(&mut self, data: &mut Cursor<Vec<u8>>) {}
}

pub struct BeesCore {
    header: BeesBlockHeader,

    major: u16,
    minor: u16,

    model: u32,

    pc: u16,
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,

    ime: u8,
    ie: u8,
    // 0 = running; 1 = halted; 2 = stopped
    execution_mode: u8,
    _padding: u8,

    io_registers: [u8; 128],

    ram: BeesBuffer,
    vram: BeesBuffer,
    mbc_ram: BeesBuffer,
    oam: BeesBuffer,
    hram: BeesBuffer,
    background_palettes: BeesBuffer,
    object_palettes: BeesBuffer,
}

impl Serialize for BeesCore {
    fn save(&self, buffer: &mut Vec<u8>) {}

    fn load(&mut self, data: &mut Cursor<Vec<u8>>) {}
}

pub fn save_state_file(file_path: &str, gb: &GameBoy) {
    let mut file = File::create(file_path).unwrap();
    let data = save_state(gb);
    file.write_all(&data).unwrap();
}

pub fn save_state(gb: &GameBoy) -> Vec<u8> {
    let mut data: Vec<u8> = vec![];

    BeesName::new(format!("{} v{}", capitalize(NAME), VERSION)).save(&mut data);

    data
}

pub fn load_state(state: Vec<u8>, gb: &GameBoy) {}
