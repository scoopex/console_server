
use std::thread;
use std::time::Duration;
use bus::Bus;
fn main() {

    let mut bus = Bus::new(5);
    let mut rx1 = bus.add_rx();
    let mut rx2 = bus.add_rx();

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        let mut count = 0;
        loop {
            println!("recv thread 1: {}, {} values received", rx1.recv().unwrap(), count);
            count += 1;
        }
    });
    thread::spawn(move || {
        let mut count = 0;
        loop {
            println!("recv thread 2: {}, {} values received", rx2.recv().unwrap(), count);
            count += 1;
        }
    });

    let mut count = 0;
    loop{
        bus.broadcast(count);
        count += 1;
        thread::sleep(Duration::from_secs(1));
    }
}


