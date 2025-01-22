use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::fmt::Display;

pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Critical => write!(f, "CRIT"),
            Priority::High => write!(f, "HIGH"),
            Priority::Medium => write!(f, "MED"),
            Priority::Low => write!(f, "LOW"),
        }
    }
}

pub struct Message {
    pub content: String,
    pub sender_id: u32,
    pub priority: Priority,
}

pub fn create_message_channel() -> (Sender<Message>, Receiver<Message>) {
    mpsc::channel()
}

pub fn create_producer_thread(messages: Vec<Message>, tx: Sender<Message>) -> JoinHandle<()> {
    thread::spawn(move || {
        messages
            .into_iter()
            .for_each(|mut msg| {
                if msg.content.contains("ERROR:") {
                    msg.priority = Priority::Critical;
                } else if msg.content.contains("WARNING:") {
                    msg.priority = Priority::High;
                } else if msg.content.contains("DEBUG:") {
                    msg.priority = Priority::Medium;
                } else {
                    msg.priority = Priority::Low;
                }
                let _ = tx.send(msg);
            }
        )
    })
}

pub fn create_consumer_thread(rx: Receiver<Message>) -> JoinHandle<Vec<String>> {
    thread::spawn(move || {
        let mut messages: Vec<String> = Vec::new();
        while let Ok(msg) = rx.recv() {
            messages.push(format!("[{}|{}] {}", msg.priority, msg.sender_id, msg.content))
        }
        messages
    })
}

// Example Usage
pub fn main() {
    let (tx, rx) = create_message_channel();

    let mut producer_handles = vec![];
    for id in 0..3 {
        let tx_clone = tx.clone();
        let messages = vec![
            Message {
                content: format!("Normal message from producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("WARNING: System running hot on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
            Message {
                content: format!("ERROR: Connection lost on producer {}", id),
                sender_id: id,
                priority: Priority::Low,
            },
        ];
        let handle = create_producer_thread(messages, tx_clone);
        producer_handles.push(handle);
    }

    drop(tx);
    let consumer_handle = create_consumer_thread(rx);

    for handle in producer_handles {
        handle.join().unwrap();
    }

    let results = consumer_handle.join().unwrap();
    println!("Processed messages:");
    for msg in results {
        println!("{}", msg);
    }
}

