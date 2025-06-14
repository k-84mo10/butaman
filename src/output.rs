use std::collections::VecDeque;
use chrono::{DateTime, FixedOffset, Utc};
use crate::SharedState;

#[derive(Clone)]
pub struct State {
    pub name: String,
    pub history: VecDeque<i32>,
    pub last_update: String,
}

pub fn update_state(state: &mut State, rtt: i32) {
    if state.history.len() == 20 {
        state.history.pop_front();
    }
    state.history.push_back(rtt);
    state.last_update = Utc::now().to_rfc3339();
}

fn rtt_to_colored_bar(rtt: i32) -> String {
    if rtt == -1 {
        return "\x1b[90m×\x1b[0m".to_string(); // Gray for timeout
    }
    let symbol = match rtt {
        r if r < 20 => "▁",
        r if r < 40 => "▂",
        r if r < 60 => "▃",
        r if r < 80 => "▄",
        r if r < 100 => "▅",
        r if r < 120 => "▆",
        r if r < 140 => "▇",
        _ => "█",
    };

    let color_code = if rtt < 40 {
        "\x1b[32m"
    } else if rtt < 100 {
        "\x1b[33m"
    } else {
        "\x1b[31m"
    };

    format!("{}{}{}", color_code, symbol, "\x1b[0m")
}

pub fn print_states(shared: &SharedState) {
    print!("\x1B[2J\x1B[H");

    let map = shared.lock().unwrap();
    println!("{:<4} {:<20} {:<30} | {:<8} | RTT   | History", "No.", "Name", "Host", "Time");

    for (i, (host, state)) in map.iter().enumerate() {
        let time_str = DateTime::parse_from_rfc3339(&state.last_update)
            .map(|dt| dt.with_timezone(&FixedOffset::east_opt(9 * 3600).unwrap()).time().format("%H:%M:%S").to_string())
            .unwrap_or_else(|_| "--:--:--".to_string());

        let latest_rtt = state.history.back().cloned().unwrap_or(-1);
        let history_str = state.history.iter().rev().map(|&rtt| rtt_to_colored_bar(rtt)).collect::<Vec<_>>().join(" ");
        println!(
            "{:<4} {:<20} {:<30} | {:<8} | {:<5} | {}",
            i + 1,
            state.name,
            host,
            time_str,
            if latest_rtt == -1 { "×".to_string() } else { format!("{}ms", latest_rtt) },
            history_str
        );
    }
}
