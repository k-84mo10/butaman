mod ping;
mod output;
mod file_loader;

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use chrono::Utc;
use tokio::time::sleep;

use ping::Pinger;
use output::{State, update_state, print_states};
use file_loader::{load_hosts_from_csv, HostEntry};

use actix_web::{web, App, HttpServer, get, Responder, HttpResponse};
use actix_cors::Cors;
use clap::Parser;

use indexmap::IndexMap;

/// CLI引数定義
#[derive(Parser)]
struct Args {
    /// Web APIモードを有効にする
    #[arg(long)]
    web: bool,
}

type SharedState = Arc<Mutex<IndexMap<String, State>>>;

/// APIエンドポイント `/api/state`
#[get("/api/state")]
async fn get_state(data: web::Data<SharedState>) -> impl Responder {
    let map = data.lock().unwrap();
    HttpResponse::Ok().json(&*map)
}

/// Webサーバ起動（APIルーティング＋pingタスク）
async fn start_web_server(hosts: Vec<HostEntry>, shared: SharedState) -> std::io::Result<()> {
    let shared_clone = shared.clone();
    tokio::spawn(async move {
        start_async(hosts, shared_clone, 1).await;
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared.clone()))
            .wrap(Cors::permissive())
            .service(get_state)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

/// pingタスク（非同期）だけを走らせる
async fn start_async(hosts: Vec<HostEntry>, shared: SharedState, interval_secs: u64) {
    for entry in hosts.clone() {
        let ip = entry.ip.clone();
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

/// pingタスク（同期スレッド）だけを走らせる
fn start_sync(hosts: Vec<HostEntry>, shared: SharedState, interval_secs: u64) {
    for (i, entry) in hosts.clone().into_iter().enumerate() {
        let ip = entry.ip.clone();
        let ssh = entry.ssh.clone();
        let shared = Arc::clone(&shared);
        let should_print = i == 0;

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

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let hosts = load_hosts_from_csv("hosts.csv");

    let shared: Arc<Mutex<IndexMap<String, State>>> = Arc::new(Mutex::new(
        hosts.iter().map(|entry| {
            (
                entry.ip.clone(),
                State {
                    name: entry.name.clone(),
                    history: std::collections::VecDeque::new(),
                    last_update: Utc::now().to_rfc3339(),
                    last_success: None,
                }
            )
        }).collect()
    ));

    if args.web {
        start_web_server(hosts, shared).await
    } else {
        start_sync(hosts, shared, 1);
        Ok(())
    }
}
