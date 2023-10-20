// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: &Queue, tx: mpsc::Sender<u32>) {
    for val in &q.first_half {
        println!("sending {:?}", val);
        tx.send(*val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    for val in &q.second_half {
        println!("sending {:?}", val);
        tx.send(*val).unwrap();
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    let queue_arc = Arc::new(queue);

    let tx_clone = mpsc::Sender::clone(&tx);
    let queue_clone = Arc::clone(&queue_arc);

    thread::spawn(move || {
        send_tx(&queue_clone, tx_clone);
    });

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        if total_received == queue_length {
            // All items received, exit the loop.
            break;
        }
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}
