use alloc::vec;
use alloc::vec::Vec;

#[repr(C)]
pub struct SuperBlock {
    pub magic: u32,
    pub total_blocks: u32,
    pub inode_count: u32,
    pub block_size: u32,
}

#[repr(C)]
pub struct Inode {
    pub size: u32,
    pub data_size: u32,
    pub blocks: [u32; 12],
    pub indirect_block: u32,
}

#[repr(C)]
pub struct DirEntry {
    pub inode: u32,
    pub name: [u8; 28],
}

const MAGIC_NUMBER: u32 = 0x12345678;

impl SuperBlock {
    pub fn new(total_blocks: u32, inode_count: u32, block_size: u32) -> Self {
        SuperBlock {
            magic: MAGIC_NUMBER,
            total_blocks,
            inode_count,
            block_size,
        }
    }
}

impl Inode {
    pub fn new(size: u32) -> Self {
        Inode {
            size,
            data_size: 0,
            blocks: [0; 12],
            indirect_block: 0,
        }
    }
}

pub struct FileSystem {
    superblock: SuperBlock,
    inodes: Vec<Inode>,
    data_blocks: Vec<Vec<u8>>,
    dir_entries: Vec<DirEntry>,
}

impl FileSystem {
    pub fn new(total_blocks: u32, inode_count: u32, block_size: u32) -> Self {
        FileSystem {
            superblock: SuperBlock::new(total_blocks, inode_count, block_size),
            inodes: Vec::new(),
            data_blocks: vec![vec![0; block_size as usize]; total_blocks as usize],
            dir_entries: Vec::new(),
        }
    }
    pub fn create_file(&mut self, size: u32, name: &str) -> Option<usize> {
        // if self.dir_entries.iter().any(|entry| {
        //     let name_str = String::from_utf8_lossy(&entry.name).trim_end_matches('\0');
        //     name_str == name
        // }) {
        //     return None;
        // }
        // let inode = Inode::new(size);
        // self.inodes.push(inode);
        // let inode_index = self.inodes.len() - 1;
        // let mut name_bytes = [0; 28];
        // let name_byes_len = usize::min(name.len(), 28);
        // name_bytes[..name_byes_len].copy_from_slice(name.as_bytes());
        // self.dir_entries.push(DirEntry {
        //     inode: inode_index as u32,
        //     name: name_bytes,
        // });
        // Some(inode_index)
        if self
            .dir_entries
            .iter()
            .any(|entry| entry.name_as_str() == name)
        {
            return None;
        }
        let inode = Inode::new(size);
        self.inodes.push(inode);
        let inodes_index = self.inodes.len() - 1;
        let mut name_bytes = [0; 28];
        let name_bytes_len = usize::min(name.len(), 28);
        name_bytes[..name_bytes_len].copy_from_slice(name.as_bytes());
        self.dir_entries.push(DirEntry {
            inode: inodes_index as u32,
            name: name_bytes,
        });
        Some(inodes_index)
    }
    pub fn write_file(&mut self, inode_index: usize, data: &[u8]) {
        // let inode = &mut self.inodes[inode_index];
        let block_size = self.superblock.block_size as usize;
        let mut offset = 0;
        let mut blocks = Vec::new();

        while offset < data.len() {
            let blocks_index = self.allocate_block();
            blocks.push(blocks_index);
            offset += block_size;
        }
        {
            let inode = &mut self.inodes[inode_index];
            for (i, &block_index) in blocks.iter().enumerate() {
                if i > inode.blocks.len() {
                    break;
                }
                inode.blocks[i] = block_index as u32;
            }
            inode.data_size = data.len() as u32;
        }
        offset = 0;
        for (_i, &block_index) in blocks.iter().enumerate() {
            let end = usize::min(offset + block_size, data.len());
            self.data_blocks[block_index][..end - offset].copy_from_slice(&data[offset..end]);
            offset += block_size;
        }
        // for i in 0..inode.blocks.len() {
        //     if offset >= data.len() {
        //         break;
        //     }
        //     let block_index = self.allocate_block();
        //     inode.blocks[i] = block_index as u32;
        //     let end = usize::min(offset + block_size, data.len());
        //     self.data_blocks[block_index][..end - offset].copy_from_slice(&data[offset..end]);
        //     offset += block_size;
        // }
    }
    pub fn read_file(&self, inode_index: usize, buffer: &mut [u8]) -> usize {
        let inode = &self.inodes[inode_index];
        let block_size = self.superblock.block_size as usize;
        let data_size = inode.data_size as usize;
        let mut offset = 0;
        for &block_index in &inode.blocks {
            if offset >= buffer.len() {
                break;
            }
            // let end = usize::min(offset + block_size, buffer.len());
            let data_block = &self.data_blocks[block_index as usize];
            let remaining_data = usize::min(buffer.len() - offset, data_size - offset);
            let copy_length = usize::min(block_size, remaining_data);
            buffer[offset..offset + copy_length].copy_from_slice(&data_block[..copy_length]);
            offset += copy_length;
        }
        offset
    }
    pub fn read_file_by_name(&self, name: &str, buffer: &mut [u8]) -> Option<usize> {
        if let Some(dir_entry) = self
            .dir_entries
            .iter()
            .find(|entry| entry.name_as_str() == name)
        {
            Some(self.read_file(dir_entry.inode as usize, buffer))
        } else {
            None
        }
    }
    pub fn allocate_block(&mut self) -> usize {
        for (i, block) in self.data_blocks.iter().enumerate() {
            if block.iter().all(|&b| b == 0) {
                return i;
            }
        }
        panic!(
            "SERIOUS FAULT: ERROR. Trying to allocate even if there are no free blocks available."
        )
    }
}

impl DirEntry {
    fn name_as_str(&self) -> &str {
        let end = self
            .name
            .iter()
            .position(|&c| c == 0)
            .unwrap_or(self.name.len());
        core::str::from_utf8(&self.name[..end]).unwrap()
    }
}
