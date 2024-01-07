use std::thread;
use crossbeam::channel;

fn main() {
    // Create a multi-producer, multi-consumer channel
    let (sender, receiver) = channel::unbounded();

    // Spawn multiple threads as receivers
    for i in 0..5 {
        let receiver = receiver.clone();
        thread::spawn(move || {
            loop {
                // Receive messages from the channel
                let msg = receiver.recv();
                match msg {
                    Ok(value) => println!("Receiver {}: Received: {}", i, value),
                    Err(_) => {
                        println!("Receiver {}: Channel closed, exiting.", i);
                        break;
                    }
                }
            }
        });
    }

    // Send messages from the single sender
    for j in 0..100000000 {
        sender.send(format!("Message {}", j)).expect("Failed to send message");
        //thread::sleep(std::time::Duration::from_millis(100));
    }

    // Close the channel to signal the receivers to exit
    drop(sender);

    // Wait for all threads to finish
    thread::sleep(std::time::Duration::from_secs(1));
}