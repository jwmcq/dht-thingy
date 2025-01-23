use dht_mmap_rust::{Dht, DhtType, Reading};

const ATTEMPTS_PER_READ: i32 = 10;
const READ_ERR_MSG: &str = "Couldn't get a reading right now!";
const OPEN_ERR_MSG: &str = "Couldn't open the sensor right now!";


pub fn read(pin: usize) -> Result<Reading, &str> {
    let mut dht = match Dht::new(DhtType::Dht11, pin) {
        Ok(d) => d,
        Err(_) => {
            return Err(OPEN_ERR_MSG)
        }
    };

    let mut reading = Err(READ_ERR_MSG);
    for _ in 0..ATTEMPTS_PER_READ {
        match dht.read() {
            Err(_) => {}
            x => { reading = x; }
        }
    };
    reading
}
