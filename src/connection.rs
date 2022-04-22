use std::fs::OpenOptions;
use std::io::Read;
use crate::{Address, ADDRESS_FILE, Connection};

pub fn get_connections() -> [Connection; 3] {
    let raw_file = OpenOptions::new().read(true).open(ADDRESS_FILE);
    let mut file = match raw_file {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let mut buff = String::new();
    file.read_to_string(&mut buff).unwrap();
    let addresses: Address = serde_json::from_str(&buff).unwrap();
    return addresses.address;
}
