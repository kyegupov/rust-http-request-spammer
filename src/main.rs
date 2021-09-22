use std::{
    sync::{Arc, RwLock},
    thread,
    time::{Duration, SystemTime},
};
use structopt::StructOpt;

#[derive(Default)]
struct Counter {
    success: u64,
    failed: u64,
    total_bytes_read: u64,
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "r", default_value = "100")]
    num_threads: usize,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let global_counter_mutex = Arc::new(RwLock::new(Counter::default()));

    for _ in 0..opt.num_threads {
        let counter_mutex = global_counter_mutex.clone();
        tokio::spawn(async move {
            let client = reqwest::Client::new();
            loop {
                let (success, count_bytes) = match client
                    .get("http://localhost")
                    .header("X-Email-Id", "someone@example.com")
                    .header("Connection", "keep-alive")
                    .send()
                    .await
                {
                    Ok(resp) => {
                        let succ = resp.status().is_success();
                        (succ, resp.bytes().await.unwrap().len())
                    }
                    Err(_) => (false, 0),
                };
                let mut c = counter_mutex.write().unwrap();
                if success {
                    c.success += 1;
                } else {
                    c.failed += 1;
                }
                c.total_bytes_read += count_bytes as u64;
            }
        });
    }
    let start = SystemTime::now();
    loop {
        thread::sleep(Duration::from_secs(10));
        let c = global_counter_mutex.read().unwrap();
        println!(
            "total successful requests: {}\ntotal failed requests: {}\nrps: {}\ntotal_bytes: {}",
            c.success,
            c.failed,
            c.success / start.elapsed().unwrap().as_secs(),
            c.total_bytes_read
        )
    }
}
