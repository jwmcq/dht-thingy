use dht_mmap_rust::{Dht, DhtType, Reading};

// const PIN: usize = 17;

const ATTEMPTS_PER_READ: i32 = 10;
const READ_ERR_MSG: &str = "Couldn't get a reading right now!";
const OPEN_ERR_MSG: &str = "Couldn't open the sensor right now!";


pub fn read(pin: usize) -> Result<Reading, String> {
    let mut dht = match Dht::new(DhtType::Dht11, pin) {
        Ok(d) => d,
        Err(_) => {
            return Err(OPEN_ERR_MSG.to_owned())
        }
    };

    let mut readings = vec![];
    for _ in 0..ATTEMPTS_PER_READ {
        match dht.read() {
            Ok(rdg) => readings.push(rdg),
            Err(_) => {}
        }
    };

    if readings.is_empty() {
        Err(READ_ERR_MSG.to_owned())
    } else {
        Ok(readings[0])
    }
}

