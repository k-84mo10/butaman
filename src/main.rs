mod ping;

use crate::ping::Pinger;
use std::thread;
use std::time::Duration;
use tokio::time::sleep;

fn ping_multiple_hosts_threaded(ips: Vec<&str>, interval_secs: u64) {
    for ip in ips {
        let ip_owned = ip.to_string();
        thread::spawn(move || {
            let pinger = Pinger::new(&ip_owned);
            loop {
                match pinger.ping_once() {
                    Ok(rtt_info) => println!("✅ {} -> {}", ip_owned, rtt_info),
                    Err(err) => eprintln!("❌ {} -> {}", ip_owned, err),
                }
                thread::sleep(Duration::from_secs(interval_secs));
            }
        });
    }

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}

async fn ping_multiple_hosts_async(ips: Vec<&str>, interval_secs: u64) {
    for ip in ips {
        let ip_owned = ip.to_string();
        tokio::spawn(async move {
            let pinger = Pinger::new(&ip_owned);
            loop {
                match pinger.ping_once() {
                    Ok(rtt_info) => println!("✅ {} -> {}", ip_owned, rtt_info),
                    Err(err) => eprintln!("❌ {} -> {}", ip_owned, err),
                }
                sleep(Duration::from_secs(interval_secs)).await;
            }
        });
    }

    loop {
        sleep(Duration::from_secs(3600)).await;
    }
}

#[tokio::main]
async fn main() {
    let hosts = vec!["203.178.135.82", "8.8.8.8", "1.1.1.1"];
    let use_async = false;

    if use_async {
        ping_multiple_hosts_async(hosts, 1).await;
    } else {
        ping_multiple_hosts_threaded(hosts, 1);
    }
}
