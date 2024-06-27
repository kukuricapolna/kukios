use crate::e1000::E1000;
use smoltcp::{
    iface::{EthernetInterfaceBuilder, NeighborCache},
    phy::{Device, DeviceCapabilities, RxToken, TxToken},
    socket::{SocketSet, TcpSocket, TcpSocketBuffer},
    time::Instant,
    wire::{EthernetAddress, IpAddress, IpCidr},
};

struct NetworkDevice {
    e1000: E1000,
}

impl Device<'_> for NetworkDevice {
    fn capabilities(&self) -> DeviceCapabilities {
        let mut caps = DeviceCapabilities::default();
        caps.max_transmission_unit = 1500;
        caps
    }
    fn transmit(&mut self, timestamp: Instant) -> Option<TxToken<'_>> {
        Some(TxToken::consume(
            timestamp,
            1500,
            |buffer: &mut [u8]| {
                let packet = b"Hello, World!";
                if packet.len() <= buffer.len() {
                    buffer[..packet.len()].copy_from_slice(packet);
                    self.e1000.send_packet(buffer);
                    Ok(packet.len())
                } else {
                    Err(smoltcp::Error::Truncated)
                }
            },
            14,
        ))
    }
    fn receive(&'_ mut self) -> Option<(Self::RxToken, Self::TxToken)> {
        None
    }
}
