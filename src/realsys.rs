extern crate alloc;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::ptr::{read_volatile, write_volatile};

use crate::println;

const BLOCK_SIZE: usize = 512;
const MAX_FILE_SIZE: usize = 1024; // 2 blocks
const DISK_BASE_ADDRESS: usize = 0x100000;

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub size: usize,
    pub blocks: [usize; 2],
}

#[derive(Debug)]
pub struct FileSystem {
    pub files: Vec<File>,
    pub free_blocks: Vec<usize>,
}

impl FileSystem {
    pub fn new() -> Self {
        FileSystem {
            files: Vec::new(),
            free_blocks: (0..100).collect(),
        }
    }

    pub fn create_file(&mut self, name: &str, data: &[u8]) -> Result<(), &str> {
        if data.len() > MAX_FILE_SIZE {
            return Err("File too large!");
        }
        let mut file = File {
            name: name.to_string(),
            size: data.len(),
            blocks: [0; 2],
        };
        for (i, chunk) in data.chunks(BLOCK_SIZE).enumerate() {
            if let Some(block) = self.free_blocks.pop() {
                file.blocks[i] = block;
                write_block(block, chunk);
            } else {
                return Err("No free blocks available.");
            }
        }
        self.files.push(file);
        Ok(())
    }

    pub fn read_file(&self, name: &str) -> Option<Vec<u8>> {
        for file in &self.files {
            if file.name == name {
                let mut data = Vec::new();
                for &block in &file.blocks {
                    if block != 0 {
                        data.extend_from_slice(&read_block(block));
                    }
                }
                data.truncate(file.size);
                return Some(data);
            }
        }
        None
    }

    pub fn write_file(&mut self, name: &str, data: &[u8]) -> Result<(), &str> {
        if data.len() > MAX_FILE_SIZE {
            return Err("File too large!");
        }
        for file in &mut self.files {
            if file.name == name {
                file.size = data.len();
                for (i, chunk) in data.chunks(BLOCK_SIZE).enumerate() {
                    write_block(file.blocks[i], chunk);
                }
                return Ok(());
            }
        }
        Err("File not found")
    }

    pub fn delete_file(&mut self, name: &str) -> Result<(), &str> {
        if let Some(pos) = self.files.iter().position(|file| file.name == name) {
            let file = self.files.remove(pos);
            for &block in &file.blocks {
                if block != 0 {
                    self.free_blocks.push(block);
                }
            }
            return Ok(());
        }
        Err("File not found")
    }
}

pub fn write_block(block: usize, data: &[u8]) {
    println!("Acknowledge of the adress.");
    let address = DISK_BASE_ADDRESS + block * BLOCK_SIZE;
    let data_len = data.len();
    let mut block_data = [0u8; BLOCK_SIZE];
    println!("Copying from slice.");
    block_data[..data_len].copy_from_slice(data);
    println!("UNSAFE");
    unsafe {
        for i in 0..BLOCK_SIZE {
            println!("Writing to volative addr: {address}, i: {i}");
            write_volatile((address + i) as *mut u8, block_data[i])
        }
    }
}

pub fn read_block(block: usize) -> [u8; BLOCK_SIZE] {
    let mut data = [0u8; BLOCK_SIZE];
    let address = DISK_BASE_ADDRESS + block * BLOCK_SIZE;
    unsafe {
        for i in 0..BLOCK_SIZE {
            data[i] = read_volatile((address + i) as *const u8)
        }
    }
    data
}
