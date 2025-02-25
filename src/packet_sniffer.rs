use pcap::{Capture, Device, Error};
use std::sync::mpsc::Sender;
use std::process;
use log::{info, error};
use chrono::Local;

fn check_npcap_installation() -> Result<(), Error> {
    if let Err(_e) = Device::list() {
        error!("Npcap not found or not properly installed. Please install Npcap from https://npcap.com/");
        process::exit(1);
    }
    info!("Npcap installation verified successfully");
    Ok(())
}

const MIKROTIK_PREFIXES: [&str; 19] = [
    "00:0C:42", "08:55:31", "18:FD:74", "2C:C8:1B", "48:8F:5A", "48:A9:8A", "4C:5E:0C", "64:D1:54",
    "6C:3B:6B", "74:4D:28", "78:9A:18", "C4:AD:34", "CC:2D:E0", "D4:01:C3", "D4:CA:6D", "DC:2C:6E",
    "E4:8D:8C", "F4:1E:57", "B8:69:F4"
];

const MIMOSA_PREFIXES: [&str; 5] = [
    "20:B5:C6", "84:9C:A4", "8C:B6:C5", "90:70:BF", "CC:54:FE"
];

const UBIQUITI_PREFIXES: [&str; 37] = [
    "00:27:22", "00:50:C2", "04:18:D6", "18:E8:29", "24:5A:4C", "24:A4:3C", "28:70:4E", "44:D9:E7",
    "60:22:32", "68:72:51", "68:D7:9A", "70:A7:41", "78:45:58", "74:83:C2", "74:AC:B9", "78:8A:20",
    "80:2A:A8", "94:2A:6F", "9C:05:D6", "AC:8B:A9", "B4:FB:E4", "D0:21:F9", "D8:B3:70", "DC:9F:DB",
    "E0:63:DA", "E4:38:83", "0C:EA:14", "1C:6A:1B", "84:78:48", "F0:9F:C2", "F4:92:BF", "F4:E2:C6",
    "FC:EC:DA", "00:15:6D", "1C:0B:8B", "58:D6:1F", "6C:63:F8"
];

fn is_mac_prefix_valid(mac: &str) -> bool {
    MIKROTIK_PREFIXES.iter().chain(MIMOSA_PREFIXES.iter()).chain(UBIQUITI_PREFIXES.iter()).any(|&prefix| mac.starts_with(prefix))
}

pub async fn start_sniffing(tx: Sender<String>) {
    info!("Starting network scan...");
    
    if let Err(e) = check_npcap_installation() {
        error!("Npcap initialization error: {}", e);
        return;
    }

    let device = Device::lookup().unwrap_or_else(|e| {
        error!("Failed to find network device: {}", e);
        process::exit(1);
    });

    info!("Using network device: {}", device.name);
    
    let mut cap = Capture::from_device(device)
        .unwrap_or_else(|e| {
            eprintln!("Failed to create capture: {}", e);
            process::exit(1);
        })
        .promisc(true)
        .open()
        .unwrap_or_else(|e| {
            eprintln!("Failed to open capture: {}", e);
            process::exit(1);
        });

    while let Ok(packet) = cap.next() {
        let mac_src = format!("{:02X}:{:02X}:{:02X}", packet[6], packet[7], packet[8]);
        let mac_dst = format!("{:02X}:{:02X}:{:02X}", packet[0], packet[1], packet[2]);

        if is_mac_prefix_valid(&mac_src) || is_mac_prefix_valid(&mac_dst) {
            let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let packet_info = format!("[{}] SRC: {} -> DST: {} | Size: {} bytes", 
            timestamp, mac_src, mac_dst, packet.len());
        info!("Packet detected: {}", packet_info);
            tx.send(packet_info).unwrap();
        }
    }
}
