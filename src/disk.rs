use core::arch::asm;

use x86_64::instructions::port::Port;

use crate::println;

pub const SECTOR_SIZE: usize = 512;

pub struct Disk;

impl Disk {
    pub fn read_sector(
        &self,
        sector: u64,
        buffer: &mut [u8; SECTOR_SIZE],
    ) -> Result<(), &'static str> {
        println!("[STATUS] Started to read the sector!");
        assert!(buffer.len() == SECTOR_SIZE);
        unsafe {
            Port::new(0x1F2).write(1_u8);
            Port::new(0x1F3).write((sector & 0xFF) as u8);
            Port::new(0x1F4).write(((sector >> 8) & 0xFF) as u8);
            Port::new(0x1F5).write(((sector >> 16) & 0xFF) as u8);
            Port::new(0x1F6).write(0xE0 | (((sector >> 24) & 0x0F) as u8));
            Port::new(0x1F7).write(0x20_u8);
        }
        self.wait_for_io();
        for i in 0..(SECTOR_SIZE / 2) {
            let data: u16 = unsafe { Port::new(0x1F0).read() };
            buffer[i * 2] = (data & 0xFF) as u8;
            buffer[i * 2 + 1] = (data >> 8) as u8;
        }
        Ok(())
    }
    pub fn write_sector(
        &self,
        sector: u64,
        buffer: &[u8; SECTOR_SIZE],
    ) -> Result<(), &'static str> {
        check_stack();
        println!("[STATUS] Started to write to sector {sector}");
        assert!(buffer.len() == SECTOR_SIZE);
        for (i, byte) in buffer.iter().enumerate() {
            println!("Buffer[{}]: {:#X}", i, byte);
        }
        unsafe {
            println!("[STATUS] Sending sector count to 0x1F2");
            Port::new(0x1F2).write(1_u8);
            io_delay();

            println!("[STATUS] Sending sector low byte to 0x1F3");
            Port::new(0x1F3).write((sector & 0xFF) as u8);
            io_delay();

            println!("[STATUS] Sending sector middle byte to 0x1F5");
            Port::new(0x1F4).write(((sector >> 8) & 0xFF) as u8);
            io_delay();

            println!("[STATUS] Sending sector high byte to 0x1F5");
            Port::new(0x1F5).write(((sector >> 16) & 0xFF) as u8);
            io_delay();

            println!("[STATUS] Sending drive/head and LBA mode to 0x1F6");
            Port::new(0x1F6).write(0xE0 | (((sector >> 24) & 0x0F) as u8));
            io_delay();

            println!("[STATUS] Sending write command to 0x1F7");
            Port::new(0x1F7).write(0x30_u8);
            io_delay();
        }
        self.wait_for_io();
        for i in 0..(SECTOR_SIZE / 2) {
            let data = u16::from(buffer[i * 2]) | (u16::from(buffer[i * 2 + 1]) << 8);
            println!("Writing data: {:#X} to port 0x1F0", data);
            unsafe {
                Port::new(0x1F0).write(data);
                io_delay()
            };
        }
        println!("[OK] Finished writing to sector {}", sector);
        io_delay();
        io_delay();
        println!("[WAIT] Still working....");
        io_delay();
        io_delay();
        println!("[OK] WORKING!");
        check_stack();
        Ok(())
    }
    fn wait_for_io(&self) {
        loop {
            println!("[WAIT] Waiting for io....");
            let status: u8 = unsafe { Port::new(0x1F7).read() };
            println!("[INFO] IO status: {:#X}", status);
            if status & 0x08 != 0 {
                println!("[OK] IO is ready!");
                break;
            }
        }
    }
}

fn io_delay() {
    println!("Delaying IO!");
    unsafe {
        core::arch::asm!("out 0x80, al", out("al") _, options(nomem, nostack, preserves_flags));
    }
    println!("IO delayed!")
}

pub fn check_stack() {
    let stack_ptr: usize;
    unsafe {
        asm!("mov {}, rsp", out(reg) stack_ptr);
    }
    println!("[STATUS] Current stack pointer: {:#X}", stack_ptr)
}
