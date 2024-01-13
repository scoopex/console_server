
use std::thread;
use std::time::Duration;
use bus::Bus;
use std::sync::{Arc,Mutex};

fn main() {

    let bus = Arc::new(Mutex::new(Bus::new(10)));

    for thread_nr in 1..10 {
        println!("Starting thread {} now", thread_nr);
        let also_bus = bus.clone();
        thread::spawn(move || {
            let mut rx1 = also_bus.lock().unwrap().add_rx();
            thread::sleep(Duration::from_secs(2));
            let mut count = 0;
            loop {
                println!("recv thread {:?}: {}, {} values received", thread::current().id(), rx1.recv().unwrap(), count);
                count += 1;
            }
        });
    }

   thread::spawn(move || {
       let mut count = 0;
       let mut real_bus = bus.lock().unwrap();
       loop {
           real_bus.broadcast(count.to_string());
           count += 1;
           thread::sleep(Duration::from_secs(1));
       }
   });
    thread::park();
}


