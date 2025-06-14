mod ping;
mod output;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use chrono::Utc;
use ping::Pinger;
use output::{State, update_state, print_states};
use tokio::time::sleep;

type SharedState = Arc<Mutex<HashMap<String, State>>>;

/// 同期バージョン
fn start_sync(hosts: Vec<(&str, Option<&str>)>, shared: SharedState, interval_secs: u64) {
    for (ip, via) in hosts.clone() {
        let ip_owned = ip.to_string();
        let via_owned = via.map(|v| v.to_string());
        let shared = Arc::clone(&shared);
        thread::spawn(move || {
            let pinger = match via_owned {
                Some(ssh_host) => Pinger::new_remote(&ssh_host, &ip_owned),
                None => Pinger::new_local(&ip_owned),
            };

            loop {
                let rtt = pinger.ping_once();
                let mut map = shared.lock().unwrap();
                if let Some(state) = map.get_mut(&ip_owned) {
                    update_state(state, rtt);
                }
                thread::sleep(Duration::from_secs(interval_secs));
            }
        });
    }

    loop {
        thread::sleep(Duration::from_secs(1));
        print_states(&shared);
    }
}

/// 非同期バージョン
async fn start_async(hosts: Vec<(&str, Option<&str>)>, shared: SharedState, interval_secs: u64) {
    for (ip, via) in hosts.clone() {
        let ip_owned = ip.to_string();
        let via_owned = via.map(|v| v.to_string());
        let shared = Arc::clone(&shared);
        tokio::spawn(async move {
            let pinger = match via_owned {
                Some(ssh_host) => Pinger::new_remote(&ssh_host, &ip_owned),
                None => Pinger::new_local(&ip_owned),
            };

            loop {
                let rtt = pinger.ping_once();
                {
                    let mut map = shared.lock().unwrap();
                    if let Some(state) = map.get_mut(&ip_owned) {
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
    let hosts = vec![
        ("203.178.135.2", None),
        ("203.178.135.82", None),
        ("203.178.135.41", None),
        ("203.178.135.87", None),
        ("juniper1", Some("hashimoto@203.178.135.65")),
        ("kohki.hongo.wide.ad.jp", None),
    ];

    let use_async = true;

    let shared: SharedState = Arc::new(Mutex::new(
        hosts
            .iter()
            .map(|(ip, _)| {
                (
                    ip.to_string(),
                    State {
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
