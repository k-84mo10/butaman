use std::process::Command;
use std::str;
use std::thread;
use std::time::Duration;

use tokio::time::sleep;

/// 単一ホストへのPingを担当する構造体
pub struct Pinger {
    pub ip: String,
}

impl Pinger {
    pub fn new(ip: &str) -> Self {
        Pinger {
            ip: ip.to_string(),
        }
    }

    /// 1回Pingを送って結果（RTT含む）を返す
    pub fn ping_once(&self) -> Result<String, String> {
        let output = Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg(&self.ip)
            .output()
            .map_err(|e| format!("Failed to execute ping: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "Ping command failed with status: {}",
                output.status
            ));
        }

        let stdout = str::from_utf8(&output.stdout)
            .map_err(|_| "Ping output not valid UTF-8".to_string())?;

        for line in stdout.lines() {
            if line.contains("time=") {
                return Ok(line.to_string());
            }
        }

        Err("RTT not found in ping output.".to_string())
    }
}

/// スレッドによる複数ホストPingループ（同期）
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

    // メインスレッドを止めず維持
    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}

/// 非同期による複数ホストPingループ（Tokio）
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

    // 非同期で永久にループ
    loop {
        sleep(Duration::from_secs(3600)).await;
    }
}

/// メイン関数
#[tokio::main]
async fn main() {
    let hosts = vec!["203.178.135.82", "8.8.8.8", "1.1.1.1"];
    let use_async = true; // ← falseにすれば同期モードに

    if use_async {
        ping_multiple_hosts_async(hosts, 1).await;
    } else {
        ping_multiple_hosts_threaded(hosts, 1);
    }
}
