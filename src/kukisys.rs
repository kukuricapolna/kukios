use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

use core::{
    ptr::write_volatile,
    slice,
    sync::atomic::{AtomicUsize, Ordering},
};

const SECTOR_SIZE: usize = 512;
const TOTAL_SECTORS: usize = 1024;

static NEXT_INODE: AtomicUsize = AtomicUsize::new(1);

#[derive(Debug, Clone)]
enum Entry {
    File(File),
    Directory(Directory),
}

#[derive(Debug, Clone)]
struct File {
    inode: usize,
    name: String,
    sectors: Vec<u64>,
}

#[repr(C, packed)]
struct DiskAddressPacket {
    size: u8,
    reserved: u8,
    count: u16,
    buffer_offset: u64,
    start_sector: u64,
}

#[derive(Debug, Clone)]
struct Directory {
    inode: usize,
    name: String,
    entries: BTreeMap<String, Entry>,
}

#[derive(Debug)]
pub struct KukiSystem {
    root: Directory,
    sector_bitmap: [u8; TOTAL_SECTORS / 8],
}

impl DiskAddressPacket {
    fn new(buffer: *mut u8, sector: u64) -> Self {
        DiskAddressPacket {
            size: core::mem::size_of::<Self>() as u8,
            reserved: 0,
            count: 1,
            buffer_offset: buffer as u64,
            start_sector: sector,
        }
    }
}

impl KukiSystem {
    pub fn new() -> Self {
        KukiSystem {
            root: Directory {
                inode: NEXT_INODE.fetch_add(1, Ordering::SeqCst),
                name: "/".to_string(),
                entries: BTreeMap::new(),
            },
            sector_bitmap: [0; TOTAL_SECTORS / 8],
        }
    }
    fn allocate_sectors(&mut self, count: usize) -> Result<Vec<u64>, &'static str> {
        let mut allocated_sectors = Vec::new();
        let mut found = 0;

        for (i, byte) in self.sector_bitmap.iter_mut().enumerate() {
            if *byte != 0xFF {
                for bit in 0..8 {
                    if *byte & (1 << bit) == 0 {
                        *byte |= 1 << bit;
                        allocated_sectors.push((i * 8 + bit) as u64);
                        found += 1;
                        if found == count {
                            return Ok(allocated_sectors);
                        }
                    }
                }
            }
        }
        for sector in &allocated_sectors {
            let byte_index = *sector as usize / 8;
            let bit_index = *sector as usize & 8;
            self.sector_bitmap[byte_index] &= !(1 << bit_index);
        }
        Err("KukiOS KukiSystem Filesystem: FATAL ERROR: Not enough free sectors!")
    }

    pub fn save(&mut self, path: &str, data: &[u8]) -> Result<(), &'static str> {
        let sectors_needed = (data.len() + SECTOR_SIZE - 1) / SECTOR_SIZE;
        let sectors = self.allocate_sectors(sectors_needed)?;
        for (i, sector) in sectors.iter().enumerate() {
            let offset = i * SECTOR_SIZE;
            let end = core::cmp::min(offset + SECTOR_SIZE, data.len());
            let slice = &data[offset..end];
            unsafe {
                if bios_disk_write(*sector, slice.as_ptr(), 1) != 0 {
                    return Err("ERROR: Disk write error!");
                }
            }
        }
        let parts: Vec<&str> = path.rsplitn(2, "/").collect();
        let dir_path = if parts.len() == 2 { parts[1] } else { "/" };
        let file_name = parts[0];
        let dir = self.get_directory_mut(dir_path)?;
        dir.entries.insert(
            file_name.to_string(),
            Entry::File(File {
                inode: NEXT_INODE.fetch_add(1, Ordering::SeqCst),
                name: file_name.to_string(),
                sectors,
            }),
        );
        Ok(())
    }
    pub fn open(&self, path: &str) -> Result<Vec<u8>, &'static str> {
        let parts: Vec<&str> = path.rsplitn(2, "/").collect();
        let dir_path = if parts.len() == 2 { parts[1] } else { "/" };
        let file_name = parts[0];
        let dir = self.get_directory(dir_path)?;
        if let Some(Entry::File(file)) = dir.entries.get(file_name) {
            let mut data = Vec::new();
            let mut buffer = [0u8; SECTOR_SIZE];
            for &sector in &file.sectors {
                unsafe {
                    if bios_disk_read(sector, buffer.as_mut_ptr(), 1) != 0 {
                        return Err("ERROR: Disk read error");
                    }
                }
                data.extend_from_slice(&buffer);
            }
            data.truncate(file.sectors.len() * SECTOR_SIZE);
            Ok(data)
        } else {
            Err("File not found!")
        }
    }
    pub fn list(&self, path: &str) -> Result<Vec<String>, &'static str> {
        let dir = self.get_directory(path).unwrap();
        let mut entries = Vec::new();

        for entry_name in dir.entries.keys() {
            entries.push(entry_name.clone());
        }
        Ok(entries)
    }
    pub fn write(&mut self, path: &str, data: &[u8]) -> Result<(), &'static str> {
        self.save(path, data)
    }
    pub fn delete(&mut self, path: &str) -> Result<(), &'static str> {
        let parts: Vec<&str> = path.rsplitn(2, "/").collect();
        let dir_path = if parts.len() == 2 { parts[1] } else { "/" };
        let file_name = parts[0];

        let dir = self.get_directory_mut(dir_path).unwrap();
        if dir.entries.remove(file_name).is_some() {
            Ok(())
        } else {
            Err("The file you want to delete, wasn't found!")
        }
    }
    fn get_directory_mut(&mut self, path: &str) -> Result<&mut Directory, &'static str> {
        let mut current = &mut self.root;
        if path == "/" {
            return Ok(current);
        }
        for part in path.split("/").filter(|p| !p.is_empty()) {
            match current.entries.get_mut(part) {
                Some(Entry::Directory(ref mut dir)) => current = dir,
                _ => return Err("Directory (mut) not found!"),
            }
        }
        Ok(current)
    }
    fn get_directory(&self, path: &str) -> Result<&Directory, &'static str> {
        let mut current = &self.root;
        if path == "/" {
            return Ok(current);
        }
        for part in path.split("/").filter(|p| !p.is_empty()) {
            match current.entries.get(part) {
                Some(Entry::Directory(ref dir)) => current = dir,
                _ => return Err("Directory not found!"),
            }
        }
        Ok(current)
    }
}

unsafe fn bios_disk_read(sector: u64, buffer: *mut u8, _count: u8) -> u8 {
    let mut dap = DiskAddressPacket::new(buffer, sector);
    let status: u8;
    write_volatile(0x80E as *mut DiskAddressPacket, dap);
    core::arch::asm!("mov ah, 0x02", "int 0x13", "setc $0", "mov $0, ah", inout(reg_byte) status => _, options(nostack, preserves_flags));
    status
}

unsafe fn bios_disk_write(sector: u64, buffer: *const u8, _count: u8) -> u8 {
    let mut dap = DiskAddressPacket::new(buffer as *mut u8, sector);
    let mut status: u8 = 0;
    write_volatile(0x80E as *mut DiskAddressPacket, dap);
    core::arch::asm!("mov ah, 0x03", "int 0x13", "setc $0", out("al") status, options(nostack, preserves_flags));
    status
}
