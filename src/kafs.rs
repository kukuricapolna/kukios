const ATA_PRIMARY_COMMAND: u16 = 0x1F0;
const ATA_PRIMARY_CONTROL: u16 = 0x3F6;
const ATA_CMD_READ_SECTORS: u8 = 0x20;
const ATA_CMD_WRITE_SECTORS: u8 = 0x30;

unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!("out dx, al", in("al") value, in("dx") port);
}

unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!("in al, dx", out("al") value, in("dx") port);
    value
}

unsafe fn outw(port: u16, value: u16) {
    core::arch::asm!("out dx, ax", in("ax") value, in("dx") port);
}

pub fn read_sector(sector: u32, buffer: &mut [u8; 512]) {
    unsafe {
        outb(
            ATA_PRIMARY_COMMAND + 6,
            0xE0 | ((sector >> 24) & 0x0F) as u8,
        );
        outb(ATA_PRIMARY_COMMAND + 1, 0);
        outb(ATA_PRIMARY_COMMAND + 2, 0);
        outb(ATA_PRIMARY_COMMAND + 3, sector as u8);
        outb(ATA_PRIMARY_COMMAND + 4, (sector >> 8) as u8);
        outb(ATA_PRIMARY_COMMAND + 5, (sector >> 16) as u8);
        outb(ATA_PRIMARY_COMMAND + 7, ATA_CMD_READ_SECTORS);
        loop {
            let status = inb(ATA_PRIMARY_COMMAND + 7);
            if status & 0x08 != 0 {
                break;
            }
        }
        for i in 0..256 {
            let word: u16;
            core::arch::asm!("in ax, dx", in("dx") ATA_PRIMARY_COMMAND, out("ax") word);
            buffer[i * 2] = word as u8;
            buffer[i * 2 + 1] = (word >> 8) as u8;
        }
    }
}

pub fn write_sector(sector: u32, buffer: &mut [u8; 512]) {
    unsafe {
        outb(
            ATA_PRIMARY_COMMAND + 6,
            0xE0 | ((sector >> 24) & 0x0F) as u8,
        );
        outb(ATA_PRIMARY_COMMAND + 1, 0);
        outb(ATA_PRIMARY_COMMAND + 2, 1);
        outb(ATA_PRIMARY_COMMAND + 3, sector as u8);
        outb(ATA_PRIMARY_COMMAND + 4, (sector >> 8) as u8);
        outb(ATA_PRIMARY_COMMAND + 5, (sector >> 16) as u8);
        outb(ATA_PRIMARY_COMMAND + 7, ATA_CMD_WRITE_SECTORS);
        loop {
            let status = inb(ATA_PRIMARY_COMMAND + 7);
            if status & 0x08 != 0 {
                break;
            }
        }
        for i in 0..256 {
            let word = (buffer[i * 2] as u16) | ((buffer[i * 2 + 1] as u16) << 8);
            outw(ATA_PRIMARY_COMMAND, word);
        }
        loop {
            let status = inb(ATA_PRIMARY_COMMAND + 7);
            if status & 0x01 != 0 {
                panic!("Error during sector write!");
            }
            if status & 0x80 == 0 {
                break;
            }
        }
    }
}
