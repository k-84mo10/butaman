mod ping;
mod output;
mod file_loader;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use chrono::Utc;
use tokio::time::sleep;

use ping::Pinger;
use output::{State, update_state, print_states};
use file_loader::{load_hosts_from_csv, HostEntry};

type SharedState = Arc<Mutex<HashMap<String, State>>>;

fn start_sync(hosts: Vec<HostEntry>, shared: SharedState, interval_secs: u64) {
    for (i, entry) in hosts.clone().into_iter().enumerate() {
        let ip = entry.ip.clone();
        let ssh = entry.ssh.clone();
        let shared = Arc::clone(&shared);

        let should_print = i == 0; // 最初のスレッドだけが表示担当

        thread::spawn(move || {
            let pinger = match ssh {
                Some(ssh_host) => Pinger::new_remote(&ssh_host, &ip),
                None => Pinger::new_local(&ip),
            };

            loop {
                let rtt = pinger.ping_once();
                {
                    let mut map = shared.lock().unwrap();
                    if let Some(state) = map.get_mut(&ip) {
                        update_state(state, rtt);
                    }
                }

                if should_print {
                    print_states(&shared);
                }

                thread::sleep(Duration::from_secs(interval_secs));
            }
        });
    }

    // メインスレッドが落ちないように待機
    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}



async fn start_async(hosts: Vec<HostEntry>, shared: SharedState, interval_secs: u64) {
    for entry in hosts.clone() {
        let ip = entry.ip.clone();
        // let name = entry.name.clone();
        let ssh = entry.ssh.clone();
        let shared = Arc::clone(&shared);

        tokio::spawn(async move {
            let pinger = match ssh {
                Some(ssh_host) => Pinger::new_remote(&ssh_host, &ip),
                None => Pinger::new_local(&ip),
            };

            loop {
                let rtt = pinger.ping_once();
                {
                    let mut map = shared.lock().unwrap();
                    if let Some(state) = map.get_mut(&ip) {
                        update_state(state, rtt);
                    }
                }
                sleep(Duration::from_secs(interval_secs)).await;
            }
        });
    }

    loop {
        sleep(Duration::from_secs(1)).await;
        print_states(&shared);
    }
}

#[tokio::main]
async fn main() {
    let hosts = load_hosts_from_csv("hosts.csv");
    let use_async = true;

    let shared: SharedState = Arc::new(Mutex::new(
        hosts
            .iter()
            .map(|entry| {
                (
                    entry.ip.clone(),
                    State {
                        name: entry.name.clone(),
                        history: std::collections::VecDeque::new(),
                        last_update: Utc::now().to_rfc3339(),
                    },
                )
            })
            .collect(),
    ));

    if use_async {
        start_async(hosts, shared, 1).await;
    } else {
        start_sync(hosts, shared, 1);
    }
}
