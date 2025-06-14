use std::process::Command;
use std::str;

pub struct Pinger {
    pub ip: String,
}

impl Pinger {
    pub fn new(ip: &str) -> Self {
        Pinger {
            ip: ip.to_string(),
        }
    }

    pub fn ping_once(&self) -> i32 {
        let output = Command::new("ping")
            .arg("-c")
            .arg("1")
            .arg("-W")
            .arg("1")
            .arg(&self.ip)
            .output();

        match output {
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
