use anyhow::{anyhow, Result};
use rand::Rng;
use std::time::Duration;
use std::{sync::mpsc, thread};

const PRODUCERS_NUM: usize = 3;

#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Msg>();

    // create a producer thread
    for idx in 0..PRODUCERS_NUM {
        let sender = tx.clone();
        thread::spawn(move || producer(sender, idx));
    }
    drop(tx);

    // create a consumer thread
    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("Received: idx:{}, value:{}", msg.idx, msg.value);
        }
        1989
    });

    let v = consumer
        .join()
        .map_err(|e| anyhow!("Consumer thread panicked: {:?}", e))?;
    println!("The consumer thread returned a secret value: {}", v);
    Ok(())
}

fn producer(tx: mpsc::Sender<Msg>, idx: usize) -> Result<()> {
    loop {
        let number: usize = rand::thread_rng().gen();
        tx.send(Msg::new(idx, number))?;
        let sleep_time: u64 = rand::thread_rng().gen_range(1..=3);
        thread::sleep(Duration::from_secs(sleep_time));
        if number % 10 == 0 {
            println!("Producer idx{} is done", idx);
            break;
        }
    }
    Ok(())
}
