use dht_mmap_rust::{Dht, DhtType};

const PIN: usize = 7;

pub fn read(pin: u8) -> Result<f32, String> {
    let mut dht = Dht::new(DhtType::Dht11, PIN).unwrap();

    for x in 1..10 {
        let reading = match dht.read() {
            Ok(rdg) => rdg.temperature(),
            Err(_) => 0.0
        };

        println!("reading pin {}", reading);
    }
    Ok(1.0)
}
