use std::{
    net::{TcpStream, ToSocketAddrs},
    time::Duration,
};

pub const MOST_COMMON_PORTS_100: &[u16] = &[
    80, 23, 443, 21, 22, 25, 3389, 110, 445, 139, 143, 53, 135, 3306, 8080, 1723, 111, 995, 993,
    5900, 1025, 587, 8888, 199, 1720, 465, 548, 113, 81, 6001, 10000, 514, 5060, 179, 1026, 2000,
    8443, 8000, 32768, 554, 26, 1433, 49152, 2001, 515, 8008, 49154, 1027, 5666, 646, 5000, 5631,
    631, 49153, 8081, 2049, 88, 79, 5800, 106, 2121, 1110, 49155, 6000, 513, 990, 5357, 427, 49156,
    543, 544, 5101, 144, 7, 389, 8009, 3128, 444, 9999, 5009, 7070, 5190, 3000, 5432, 1900, 3986,
    13, 1029, 9, 5051, 6646, 49157, 1028, 873, 1755, 2717, 4899, 9100, 119, 37,
];

pub fn scan_port_one(ip: String, port: u16) -> u16 {
    // let mut open_port = vec![];
    let socket = format!("{}:{}", ip, port)
        .to_socket_addrs()
        .unwrap()
        .next()
        .unwrap();
    let timeout = Duration::from_secs(3);
    if TcpStream::connect_timeout(&socket, timeout).is_ok() {
        return port;
    } else {
        return 0 as u16;
    }
}
