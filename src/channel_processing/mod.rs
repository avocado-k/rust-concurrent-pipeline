use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn main() {
    let (tx, rx) = mpsc::channel();
    let num_producers = 4;

    for i in 0..num_producers {
        let tx = tx.clone();
        thread::spawn(move || {
            for j in 0..5 {
                let msg = format!("Producer {} - Message {}", i, j);
                tx.send(msg.clone()).unwrap();
                //tx.send(msg).unwrap();
                println!("Sent: {}", msg);
                thread::sleep(Duration::from_secs(1));
            }
        });
    }

    drop(tx);

    let mut received_messages = Vec::new();
    for msg in rx {
        println!("Received: {}", msg);
        received_messages.push(msg);
    }

    println!("All messages received:");
    for msg in received_messages {
        println!("{}", msg);
    }
}

#[test]
fn test_mpsc_channel() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        tx.send("Test Message".to_string()).unwrap();
    });
    handle.join().unwrap();
    let received = rx.recv().unwrap();
    assert_eq!(received, "Test Message");
}