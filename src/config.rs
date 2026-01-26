use core::net::Ipv4Addr;

pub const IP_ADDRESS: Ipv4Addr = Ipv4Addr::new(192, 168, 50, 40);
pub const IP_ADDRESS_PREFIX: u8 = 24;
pub const PORT: u16 = 8050;
pub const GATEWAY: Ipv4Addr = Ipv4Addr::new(192, 168, 50, 1);
pub const MAC_ADDRESS: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x00];
