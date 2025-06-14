use std::process::Command;
use std::str;

pub struct Pinger {
    pub target_ip: String,
    pub via_ssh: Option<String>, // Some("user@host") or None
}

impl Pinger {
    /// ローカルから直接ping
    pub fn new_local(ip: &str) -> Self {
        Pinger {
            target_ip: ip.to_string(),
            via_ssh: None,
        }
    }

    /// SSH経由でping
    pub fn new_remote(ssh_host: &str, target_ip: &str) -> Self {
        Pinger {
            target_ip: target_ip.to_string(),
            via_ssh: Some(ssh_host.to_string()),
        }
    }

    /// pingを1回だけ実行し、RTT（整数ms）を返す。失敗時は -1。
    pub fn ping_once(&self) -> i32 {
        let command = if let Some(ssh_host) = &self.via_ssh {
            // SSH経由で実行: ssh user@host "ping -c 1 -W 1 target_ip"
            Command::new("ssh")
                .arg(ssh_host)
                .arg(format!("ping -c 1 -W 1 {}", self.target_ip))
                .output()
        } else {
            // ローカルで実行: ping -c 1 -W 1 target_ip
            Command::new("ping")
                .arg("-c")
                .arg("1")
                .arg("-W")
                .arg("1")
                .arg(&self.target_ip)
                .output()
        };

        match command {
            Ok(output) => {
                if output.status.success() {
                    let stdout = str::from_utf8(&output.stdout).unwrap_or("");
                    for line in stdout.lines() {
                        if let Some(start) = line.find("time=") {
                            let time_part = &line[start + 5..];
                            if let Some(end) = time_part.find(" ms") {
                                let rtt = &time_part[..end];
                                return rtt.parse::<f32>().unwrap_or(-1.0) as i32;
                            }
                        }
                    }
                    -1
                } else {
                    -1
                }
            }
            Err(_) => -1,
        }
    }
}
