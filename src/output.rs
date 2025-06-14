use std::collections::VecDeque;

use chrono::Utc;
use crate::SharedState;

#[derive(Clone)]
pub struct State {
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
    let symbol = if rtt == -1 {
        return "\x1b[90m×\x1b[0m".to_string(); // Gray for timeout
    } else {
        // スケール設定（1段階20msとする）
        let scale = 20;
        match rtt {
            r if r < scale * 1 => "▁",
            r if r < scale * 2 => "▂",
            r if r < scale * 3 => "▃",
            r if r < scale * 4 => "▄",
            r if r < scale * 5 => "▅",
            r if r < scale * 6 => "▆",
            r if r < scale * 7 => "▇",
            _ => "█",
        }
    };

    // 色付け
    let color_code = if rtt < 40 {
        "\x1b[32m" // Green
    } else if rtt < 100 {
        "\x1b[33m" // Yellow
    } else {
        "\x1b[31m" // Red
    };

    format!("{}{}{}", color_code, symbol, "\x1b[0m")
}

pub fn print_states(shared: &SharedState) {
    print!("\x1B[2J\x1B[H"); // ANSI escape code to clear screen

    let map = shared.lock().unwrap();
    println!(
        "{:<4} {:<30} | {:<24} | History",
        "No.", "Host", "Last Update"
    );

    for (i, (host, state)) in map.iter().enumerate() {
        let history_str = state
            .history
            .iter()
            .map(|&rtt| rtt_to_colored_bar(rtt))
            .collect::<Vec<_>>()
            .join(" ");
        println!(
            "{:<4} {:<30} | {:<24} | {}",
            i + 1,
            host,
            state.last_update,
            history_str
        );
    }
    println!();
}
