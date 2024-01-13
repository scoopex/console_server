
use std::thread;
use std::time::Duration;
use bus::Bus;
use std::sync::{Arc,Mutex};

#[derive(Clone)]
pub struct ConsoleActivity {
    pub body: String,
}
fn main() {

    let bus: Bus<ConsoleActivity> = Bus::new(10);
    let protected_bus = Arc::new(Mutex::new(bus));

    for thread_nr in 1..10 {
        println!("Starting thread {} now", thread_nr);
        let also_bus = protected_bus.clone();
        thread::spawn(move || {
            let mut rx1 = also_bus.lock().unwrap().add_rx();
            thread::sleep(Duration::from_secs(2));
            let mut count = 0;
            loop {
                let event = rx1.recv().unwrap();
                println!("recv thread {:?}: {}, {} values received", thread::current().id(), event.body, count);
                count += 1;
            }
        });
    }

   thread::spawn(move || {
       let mut count = 0;
       let mut bus = protected_bus.lock().unwrap();
       loop {
           let event = ConsoleActivity{
               body: count.to_string(),
           };
           bus.broadcast(event);
           count += 1;
           thread::sleep(Duration::from_millis(100));
       }
   });
    thread::park();
}


