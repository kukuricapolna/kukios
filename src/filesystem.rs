use alloc::{borrow::ToOwned, vec::Vec};

use spin::Mutex;

use crate::{
    disk::{Disk, SECTOR_SIZE},
    println,
};

pub const BLOCK_SIZE: usize = SECTOR_SIZE;
pub const TOTAL_BLOCKS: usize = 1024; // 512KB Filesystem
pub const MAX_FILES: usize = 128;

static FILE_SYSTEM: Mutex<FileSystem> = Mutex::new(FileSystem::new());

#[repr(C)]
pub struct SuperBlock {
    magic: u32,
    total_blocks: u32,
    free_blocks: u32,
    root_dir_block: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Inode {
    size: u32,
    block_ptrs: [u32; 10],
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DirectoryEntry {
    name: [u8; 32],
    inode: u32,
}

pub struct Directory {
    entries: [DirectoryEntry; MAX_FILES],
}

#[allow(dead_code)]
pub struct FileSystem {
    superblock: SuperBlock,
    inodes: [Inode; TOTAL_BLOCKS],
    root_directory: Directory,
}

impl FileSystem {
    pub const fn new() -> Self {
        Self {
            superblock: SuperBlock {
                magic: 0xF0F03410,
                total_blocks: TOTAL_BLOCKS as u32,
                free_blocks: TOTAL_BLOCKS as u32 - 1,
                root_dir_block: 1,
            },
            inodes: [Inode {
                size: 0,
                block_ptrs: [0; 10],
            }; TOTAL_BLOCKS],
            root_directory: Directory {
                entries: [DirectoryEntry {
                    name: [0; 32],
                    inode: 0,
                }; MAX_FILES],
            },
        }
    }
    pub fn init(&mut self) {
        println!("Initing....");
        self.inodes[0].size = 0;
        self.inodes[0].block_ptrs[0] = 1;
    }
    pub fn read_block(&self, block_num: usize) -> [u8; BLOCK_SIZE] {
        println!("Reading block....");
        let mut buffer = [0u8; BLOCK_SIZE];
        Disk::read_sector(&Disk, block_num as u64, &mut buffer)
            .unwrap_or_else(|err| println!("Error reading sector: {err}"));
        buffer
    }
    pub fn write_block(&mut self, block_num: usize, data: &[u8]) {
        println!("Writing block {block_num}....");
        assert!(data.len() == BLOCK_SIZE);
        println!("Let buffer");
        let mut buffer = [0u8; BLOCK_SIZE];
        println!("Let buffer copy from slice");
        buffer.copy_from_slice(data);
        println!("Write sector!");
        match Disk::write_sector(&Disk, block_num as u64, &mut &buffer) {
            Ok(_) => {
                println!("[OK] Finished writing sector!");
            }
            Err(err) => {
                println!("Error: {err}");
            }
        };
        println!("Finished writing block....");
        println!("Finished writing block {block_num}");
    }
    pub fn read_inode(&self, inode_num: usize) -> &Inode {
        println!("Reading inode....");
        &self.inodes[inode_num]
    }
    pub fn write_inode(&mut self, inode_num: usize, inode: Inode) {
        println!("Writing inode....");
        self.inodes[inode_num] = inode;
    }
    pub fn create_file(&mut self, name: &str) -> Result<u32, &'static str> {
        println!("Creating file: {name}");
        let mut free_inode = None;
        for (i, inode) in self.inodes.iter_mut().enumerate() {
            println!("For inode");
            if inode.size == 0 {
                free_inode = Some(i as u32);
                break;
            }
        }
        let inode_num = match free_inode {
            Some(num) => num,
            None => return Err("No free inodes available!"),
        };
        let mut free_entry = None;
        for entry in self.root_directory.entries.iter_mut() {
            println!("For entry");
            if entry.name[0] == 0 {
                free_entry = Some(entry);
                break;
            }
        }
        let entry = match free_entry {
            Some(e) => e,
            None => return Err("No free directory entries available."),
        };
        let name_bytes = name.as_bytes();
        if name_bytes.len() > 32 {
            return Err("File name longer than 32 chars / bytes!");
        }
        println!("Last part of creating the file");
        entry.name[..name_bytes.len()].copy_from_slice(name_bytes);
        entry.inode = inode_num;
        println!("File {name} created with inode {inode_num}");
        Ok(inode_num)
    }
    fn find_free_blocks(&self, count: usize) -> Result<Vec<u32>, &'static str> {
        println!("Finding free blocks");
        let mut free_blocks = Vec::new();
        for block_num in 1..TOTAL_BLOCKS {
            if self
                .inodes
                .iter()
                .all(|inode| !inode.block_ptrs.contains(&(block_num as u32)))
            {
                free_blocks.push(block_num as u32);
                if free_blocks.len() == count {
                    break;
                }
            }
        }
        if free_blocks.len() == count {
            Ok(free_blocks)
        } else {
            Err("Not enough free blocks available!")
        }
    }
    pub fn write_file(&mut self, inode_num: u32, data: &[u8]) -> Result<(), &'static str> {
        println!("Writing file to inode: {inode_num}");
        if inode_num as usize >= TOTAL_BLOCKS {
            return Err("Invalid inode number!");
        }
        let blocks_needed = (data.len() + BLOCK_SIZE - 1) / BLOCK_SIZE;
        if blocks_needed > 10 {
            return Err("File too large!"); // TODO: Better error message.
        }
        let free_blocks = self.find_free_blocks(blocks_needed)?;
        for (i, &block_num) in free_blocks.iter().enumerate() {
            let start = i * BLOCK_SIZE;
            let end = core::cmp::min(start + BLOCK_SIZE, data.len());
            let block_data = &data[start..end];
            let mut block_buffer = [0u8; BLOCK_SIZE];

            println!(
                "Copying data to block buffer, start: {}, end: {}",
                start, end
            );
            // println!("1 important");
            block_buffer[..block_data.len()].copy_from_slice(block_data);
            // println!("2 important");
            println!(
                "Writing block number {} with data {:?}",
                block_num,
                &block_buffer[..]
            );
            self.write_block(block_num as usize, &block_buffer);
            // println!("3 important");
            println!(
                "Updating inode block pointers, inode_num: {}, i: {}, block_num: {}",
                inode_num, i, block_num
            );
            if i < self.inodes[inode_num as usize].block_ptrs.len() {
                self.inodes[inode_num as usize].block_ptrs[i] = block_num;
            } else {
                println!("THE I IS {i}");
                return Err("Block pointer index out of bounds");
            }
        }
        println!(
            "Setting inode size, inode_num: {}, size: {}",
            inode_num,
            data.len()
        );
        self.inodes[inode_num as usize].size = data.len() as u32;
        let inode = &self.inodes[inode_num as usize];
        println!(
            "Inode details: size: {}, block_ptrs: {:?}",
            inode.size, inode.block_ptrs
        );
        println!(
            "Finished writing file. Inode size: {}",
            self.inodes[inode_num as usize].size
        );
        Ok(())
    }
}

pub fn read_block(block_num: usize) -> [u8; BLOCK_SIZE] {
    FILE_SYSTEM.lock().read_block(block_num)
}

pub fn write_block(block_num: usize, data: &[u8]) {
    FILE_SYSTEM.lock().write_block(block_num, data)
}

pub fn read_inode(inode_num: usize) -> Inode {
    FILE_SYSTEM.lock().read_inode(inode_num).to_owned()
}

pub fn write_inode(inode_num: usize, inode: Inode) {
    FILE_SYSTEM.lock().write_inode(inode_num, inode)
}

pub fn init_fs() {
    FILE_SYSTEM.lock().init();
}

pub fn create_file(name: &str) -> Result<u32, &'static str> {
    FILE_SYSTEM.lock().create_file(name)
}

pub fn write_file(inode_num: u32, data: &[u8]) -> Result<(), &'static str> {
    FILE_SYSTEM.lock().write_file(inode_num, data)
}
