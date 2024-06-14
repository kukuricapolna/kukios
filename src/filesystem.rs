use alloc::string::{String, ToString};
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
            blocks: [0; 12],
            indirect_block: 0,
        }
    }
}

pub struct FileSystem {
    superblock: SuperBlock,
    inodes: Vec<Inode>,
    data_blocks: Vec<Vec<u8>>,
}

impl FileSystem {
    pub fn new(total_blocks: u32, inode_count: u32, block_size: u32) -> Self {
        FileSystem {
            superblock: SuperBlock::new(total_blocks, inode_count, block_size),
            inodes: Vec::new(),
            data_blocks: vec![vec![0; block_size as usize]; total_blocks as usize],
        }
    }
    pub fn create_file(&mut self, size: u32) -> usize {
        let inode = Inode::new(size);
        self.inodes.push(inode);
        self.inodes.len() - 1
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
        }
        offset = 0;
        for (i, &block_index) in blocks.iter().enumerate() {
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
        let mut offset = 0;
        for &block_index in &inode.blocks {
            if offset >= buffer.len() {
                break;
            }
            // let end = usize::min(offset + block_size, buffer.len());
            let data_block = &self.data_blocks[block_index as usize];
            let remaining_data = buffer.len() - offset;
            let copy_length = usize::min(block_size, remaining_data);
            buffer[offset..offset + copy_length].copy_from_slice(&data_block[..copy_length]);
            offset += copy_length;
        }
        offset
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
