use core::arch::asm;

use x86_64::instructions::port::{Port, PortGeneric, PortReadOnly, PortWriteOnly};

const ATA_DATA_REG: u16 = 0x1F0;
const ATA_SECTOR_COUNT_REG: u16 = 0x1F2;
const ATA_LBA_LOW_REG: u16 = 0x1F3;
const ATA_LBA_MID_REG: u16 = 0x1F4;
const ATA_LBA_HIGH_REG: u16 = 0x1F5;
const ATA_DRIVE_HEAD_REG: u16 = 0x1F6;
const ATA_COMMAND_REG: u16 = 0x1F7;
const ATA_STATUS_REG: u16 = 0x1F7;

const ATA_STATUS_BSY: u8 = 0b1000_0000;
const ATA_STATUS_DRQ: u8 = 0b0000_1000;

fn initialize_disk() {
    unsafe {
        let mut drive_head_port = PortWriteOnly::new(ATA_DRIVE_HEAD_REG);
        drive_head_port.write(0xA0);
        // TODO: WAIT FOR DISK!
        let mut status_port = PortReadOnly::new(ATA_STATUS_REG);
        status_port.read();
        let mut command_port = PortWriteOnly::new(ATA_COMMAND_REG);
        command_port.write(0xEC);
    }
}

fn wait_for_disk_ready() {
    unsafe {
        let mut status_port = PortReadOnly::new(ATA_STATUS_REG);
        while (status_port.read() & ATA_STATUS_BSY) != 0 {
            // wait
            asm!("nop")
        }
    }
}

fn sector_read(sector: u32, buffer: &mut [u8]) {
    unsafe {
        let sector_cound = 1;
        let lba = sector;
        let mut drive_head_port = PortWriteOnly::<u8>::new(ATA_DRIVE_HEAD_REG);
        drive_head_port.write(0xE0 | ((lba >> 24) & 0x0F) as u8);
        let mut sector_count_port = PortWriteOnly::new(ATA_SECTOR_COUNT_REG);
        sector_count_port.write(sector_cound as u8);
        let mut lba_low_port = PortWriteOnly::<u8>::new(ATA_LBA_LOW_REG);
        lba_low_port.write((lba 0xFF) as u8);
        let mut lba_mid_port = PortWriteOnly::new(ATA_LBA_MID_REG);
        lba_mid_port.write(((lba >> 8) & 0xFF) as u8);
        let mut lba_high_port = PortWriteOnly::new(ATA_LBA_HIGH_REG);
        lba_high_port.write(((lba >> 16) & 0xFF) as u8);
        let mut command_port = PortWriteOnly::<u8>::new(ATA_COMMAND_REG);
        command_port.write(0x20);
        wait_for_disk_ready();
        let mut data_port = Port::new(ATA_DATA_REG);
        let r = data_port.read();
    }
}

fn sector_write(sector: u32, data: &[u8]) {
    unsafe {
        let sector_count = 1;
        let lba = sector;
        let mut drive_head_port = PortWriteOnly::new(ATA_DRIVE_HEAD_REG);
        drive_head_port.write(0xE0 | ((lba >> 24) & 0x0F) as u8);
        let mut sector_count_port = PortWriteOnly::new(ATA_SECTOR_COUNT_REG);
        sector_count_port.write(sector_count as u8);
        let mut lba_low_port = PortWriteOnly::new(ATA_LBA_LOW_REG);
        lba_low_port.write(((lba >> 8) & 0xFF) as u8);
        let mut lba_high_port = PortWriteOnly::new(ATA_LBA_HIGH_REG);
        lba_high_port.write(((lba >> 16) & 0xFF) as u8);
        let mut command_port = PortWriteOnly::new(ATA_COMMAND_REG);
        command_port.write(0x30);
        wait_for_disk_ready();
        let mut data_port = Port::<u16>::new(ATA_DATA_REG);
        let data_to_write: u16 = data;
        data_port.write(data_to_write);
    }
}
