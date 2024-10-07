// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



use std::sync::{mpsc, Arc, Mutex};
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

fn send_tx(q: Queue, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    // 创建线程来发送第一个半部分的值
    let first_half_thread = thread::spawn({
        let tx = Arc::clone(&tx); // 克隆 Arc，以便在闭包中使用
        move || {
            for val in &q.first_half {
                println!("sending {:?}", val);
                let _ = tx.lock().unwrap().send(*val); // 获取锁并发送
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    // 创建线程来发送第二个半部分的值
    let second_half_thread = thread::spawn({
        let tx = Arc::clone(&tx); // 克隆 Arc，以便在闭包中使用
        move || {
            for val in &q.second_half {
                println!("sending {:?}", val);
                let _ = tx.lock().unwrap().send(*val); // 获取锁并发送
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    // 等待两个线程完成
    first_half_thread.join().unwrap();
    second_half_thread.join().unwrap();
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    // 使用 Arc 和 Mutex 包装 Sender
    let tx = Arc::new(Mutex::new(tx));
    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}


