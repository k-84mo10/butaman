use std::fs::File;
use std::io::BufReader;
use csv::ReaderBuilder;

#[derive(Debug, Clone)]
pub struct HostEntry {
    pub name: String,
    pub ip: String,
    pub ssh: Option<String>,
}

pub fn load_hosts_from_csv(path: &str) -> Vec<HostEntry> {
    let file = File::open(path).expect("CSVファイルを開けませんでした");
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));

    let mut hosts = Vec::new();

    for result in reader.records() {
        if let Ok(record) = result {
            let name = record.get(0).unwrap_or("").trim().to_string();
            let ip = record.get(1).unwrap_or("").trim().to_string();
            let ssh = record.get(2).unwrap_or("").trim().to_string();

            if !ip.is_empty() {
                hosts.push(HostEntry {
                    name,
                    ip,
                    ssh: if ssh.is_empty() { None } else { Some(ssh) },
                });
            }
        }
    }

    hosts
}
