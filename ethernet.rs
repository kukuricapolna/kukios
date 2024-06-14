use core::ptr;

use smoltcp::phy::{DeviceCapabilities, RxToken, TxToken};

const IO_BASE: u16 = 0xC000;

pub struct EthernetDriver;

impl EthernetDriver {
    pub fn new() -> Self {
        EthernetDriver
    }
    pub fn init(&self) {
        unsafe {
            ptr::write_volatile(IO_BASE as *mut u8, 0x00);
        }
    }
    pub fn send_packet(&self, data: &[u8]) {
        for &byte in data {
            unsafe { ptr::write_volatile(IO_BASE as *mut u8, byte) }
        }
    }
    pub fn receive_packet(&self) -> Option<&[u8]> {
        None
    }
}

pub struct MyDevice;

impl Device for MyDevice {
    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = 1500;
        caps
    }
    fn transmit(&mut self) -> Option<TxToken> {
        None
    }
    fn receive(&mut self) -> Option<RxToken> {
        None
    }
}
