use anyhow::Result;
use concurrency::AmapMetrics;
use rand::Rng;
use std::{thread, time::Duration};

const NUMBER_THREADS: usize = 5;
const METRICS_NAME: [&str; 10] = [
    "api.v1.task1",
    "api.v1.task2",
    "api.v1.task3",
    "api.v1.task4",
    "api.v1.task5",
    "api.v1.server1",
    "api.v1.server2",
    "api.v1.server3",
    "api.v1.server4",
    "api.v1.server5",
];

fn main() {
    let metrics = AmapMetrics::new(&METRICS_NAME);

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

fn task_worker(idx: usize, metrics: AmapMetrics) -> Result<()> {
    thread::spawn(move || -> Result<()> {
        loop {
            metrics.inc(format!("api.v1.task{}", idx))?;

            let sleep_time = rand::thread_rng().gen_range(500..=2000);
            thread::sleep(Duration::from_millis(sleep_time));
        }
    });
    Ok(())
}

fn server_worker(metrics: AmapMetrics) -> Result<()> {
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
