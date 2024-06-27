use x86_64::instructions::port::Port;

const E1000_REG_CTRL: u16 = 0x0000;
const E1000_REG_STATUS: u16 = 0x0008;
const E1000_REG_EECD: u16 = 0x0010;
const E1000_REG_TCTL: u16 = 0x0400;
const E1000_REG_RCTL: u16 = 0x0100;
const E1000_REG_TDBAL: u16 = 0x3800;
const E1000_REG_TDBAH: u16 = 0x3804;
const E1000_REG_TDLEN: u16 = 0x3808;
const E1000_REG_TDH: u16 = 0x3810;
const E1000_REG_TDT: u16 = 0x3818;

const NUM_TX_DESC: u16 = 8;
const TX_BUFFER_SIZE: usize = 2048;

#[repr(C, packed)]
struct E1000TxDesc {
    buffer_addr: u64,
    length: u16,
    cso: u8,
    cmd: u8,
    status: u8,
    css: u8,
    special: u16,
}

struct TxDescriptorRing {
    descriptors: [E1000TxDesc; NUM_TX_DESC],
    buffers: [[u8; TX_BUFFER_SIZE]; NUM_TX_DESC],
}

impl TxDescriptorRing {
    pub const fn new() -> Self {
        const DESC: E1000TxDesc = E1000TxDesc {
            buffer_addr: 0,
            length: 0,
            cso: 0,
            cmd: 0,
            status: 0,
            css: 0,
            special: 0,
        };
        const BUFFER: [u8; TX_BUFFER_SIZE] = [0; TX_BUFFER_SIZE];
        TxDescriptorRing {
            descriptors: [DESC; NUM_TX_DESC],
            buffers: [BUFFER; NUM_TX_DESC],
        }
    }
}

pub struct E1000 {
    base: u16,
    tx_ring: TxDescriptorRing,
    tx_next_desc: usize,
}

impl E1000 {
    pub fn new(base: u16) -> Self {
        Self {
            base,
            tx_ring: TxDescriptorRing::new(),
            tx_next_desc: 0,
        }
    }
    pub fn init(&mut self) {
        unsafe {
            let mut ctrl = Port::<u32>::new(self.base + E1000_REG_CTRL);
            ctrl.write(0x04000000);
            let mut rctl = Port::<u32>::new(self.base + E1000_REG_RCTL);
            rctl.write(0x00000002);
            let mut tctl = Port::<u32>::new(self.base + E1000_REG_TCTL);
            tctl.write(0x00000002);

            let tx_ring_ptr = &self.tx_ring as *const _ as u64;
            let mut tdbal = Port::<u32>::new(self.base + E1000_REG_TDBAL);
            tdbal.write((tx_ring_ptr & 0xFFFF_FFFF) as u32);
            let mut tdbah = Port::<u32>::new(self.base + E1000_REG_TDBAH);
            tdbah.write((tx_ring_ptr >> 32) as u32);
            let mut tdlen = Port::<u32>::new(self.base + E1000_REG_TDLEN);
            tdlen.write((NUM_TX_DESC * core::mem::size_of::<E1000TxDesc>()) as u32);
            let mut tdh = Port::<u32>::new(self.base + E1000_REG_TDH);
            tdh.write(0);
            let mut tdt = Port::<u32>::new(self.base + E1000_REG_TDT);
            tdt.write(0);

            for i in 0..NUM_TX_DESC {
                self.tx_ring.descriptors[i].buffer_addr =
                    &self.tx_ring.buffers[i] as *const _ as u64;
                self.tx_ring.descriptors[i].status = 0x1; //DESCRIPTOR AVAILABLE
            }
        }
    }
    pub fn send_packet(&mut self, packet: &[u8]) {
        let current_desc = self.tx_next_desc;
        if packet.len() > TX_BUFFER_SIZE {
            return;
        }
        self.tx_ring.buffers[current_desc][..packet.len()].copy_from_slice(packet);
        self.tx_ring.descriptors[current_desc].length = packet.len() as u16;
        self.tx_ring.descriptors[current_desc].cmd = 0b1000_1000; // RS and EOP
        self.tx_ring.descriptors[current_desc].status = 0;
        self.tx_next_desc = (self.tx_next_desc ! + 1) % NUM_TX_DESC;

        unsafe {
            let mut tdt = Port::<u32>::new(self.base + E1000_REG_TDT);
            tdt.write(self.tx_next_desc as u32);
        }
    }
    pub fn receive_packet(&self, buffer: &mut [u8]) -> usize {
        todo!()
    }
}
