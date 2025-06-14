use std::process::Command;
use std::str;

pub struct Pinger {
    pub target_ip: String,
    pub via_ssh: Option<String>, // Some("user@host") or None
}

impl Pinger {
    pub fn new_local(ip: &str) -> Self {
        Pinger {
            target_ip: ip.to_string(),
            via_ssh: None,
        }
    }

    pub fn new_remote(ssh_host: &str, target_ip: &str) -> Self {
        Pinger {
            target_ip: target_ip.to_string(),
            via_ssh: Some(ssh_host.to_string()),
        }
    }

    /// IPv6かどうかを判定（簡易判定：コロンが含まれるか）
    fn is_ipv6(&self) -> bool {
        self.target_ip.contains(':')
    }

    /// pingを1回だけ実行し、RTT（整数ms）を返す。失敗時は -1。
    pub fn ping_once(&self) -> i32 {
        let use_ipv6 = self.is_ipv6();

        let command = if let Some(ssh_host) = &self.via_ssh {
            // SSH経由: ping or ping -6 を選択
            let cmd = if use_ipv6 {
                format!("ping -6 -c 1 -W 1 {}", self.target_ip)
            } else {
                format!("ping -c 1 -W 1 {}", self.target_ip)
            };
            Command::new("ssh").arg(ssh_host).arg(cmd).output()
        } else {
            // ローカル実行: ping or ping -6 を選択
            let mut cmd = Command::new("ping");
            if use_ipv6 {
                cmd.arg("-6");
            }
            cmd.arg("-c")
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
