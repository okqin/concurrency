use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
use std::{thread, time::Duration};

const NUMBER_THREADS: usize = 5;

fn main() {
    let metrics = Metrics::new();

    println!("{}", metrics);

    for i in 1..=NUMBER_THREADS {
        if let Err(e) = task_worker(i, metrics.clone()) {
            eprintln!("task_worker Error: {:?}", e)
        }
    }

    for _ in 0..NUMBER_THREADS {
        if let Err(e) = server_worker(metrics.clone()) {
            eprintln!("server_worker Error: {:?}", e)
        }
    }

    loop {
        println!("{}", metrics);
        thread::sleep(Duration::from_secs(1));
    }
}

fn task_worker(idx: usize, metrics: Metrics) -> Result<()> {
    thread::spawn(move || -> Result<()> {
        loop {
            metrics.inc(format!("api.v1.task{}", idx))?;

            let sleep_time = rand::thread_rng().gen_range(500..=2000);
            thread::sleep(Duration::from_millis(sleep_time));
        }
    });
    Ok(())
}

fn server_worker(metrics: Metrics) -> Result<()> {
    thread::spawn(move || -> Result<()> {
        loop {
            let number = rand::thread_rng().gen_range(1..=5);
            metrics.inc(format!("api.v1.server{}", number))?;

            let sleep_time = rand::thread_rng().gen_range(100..=1000);
            thread::sleep(Duration::from_millis(sleep_time));
        }
    });
    Ok(())
}
