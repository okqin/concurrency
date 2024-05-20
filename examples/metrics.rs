use anyhow::Result;
use concurrency::Metrics;
use rand::Rng;
use std::{collections::HashMap, thread, time::Duration};

const NUMBER_THREADS: usize = 5;

fn main() {
    let metrics = Metrics::new();

    let metrics_data = get_metrics_data(&metrics);
    println!("{:?}", metrics_data);

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
        let metrics_data = get_metrics_data(&metrics);
        println!("{:?}", metrics_data);
        thread::sleep(Duration::from_secs(2));
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

fn get_metrics_data(metrics: &Metrics) -> HashMap<String, i64> {
    match metrics.snapshot() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("metrics.snapshot() Error: {:?}", e);
            HashMap::default()
        }
    }
}
