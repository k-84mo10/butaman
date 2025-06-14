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
