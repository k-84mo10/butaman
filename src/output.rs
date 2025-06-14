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
    print!("\x1B[2J\x1B[H"); // 画面クリア

    let map = shared.lock().unwrap();
    println!(
        "{:<4} {:<30} | {:<8} | {:<8} | History",
        "No.", "Host", "Time", "RTT(ms)"
    );

    for (i, (host, state)) in map.iter().enumerate() {
        // JST に変換
        let time_str = chrono::DateTime::parse_from_rfc3339(&state.last_update)
            .map(|dt| {
                dt.with_timezone(&chrono::FixedOffset::east_opt(9 * 3600).unwrap())
                    .time()
                    .format("%H:%M:%S")
                    .to_string()
            })
            .unwrap_or_else(|_| "--:--:--".to_string());

        // 最新のRTTを取得
        let rtt_str = match state.history.back() {
            Some(&-1) | None => "--".to_string(),
            Some(&val) => format!("{}", val),
        };

        // 棒グラフ（逆順）
        let history_str = state
            .history
            .iter()
            .rev()
            .map(|&rtt| rtt_to_colored_bar(rtt))
            .collect::<Vec<_>>()
            .join(" ");

        println!(
            "{:<4} {:<30} | {:<8} | {:<8} | {}",
            i + 1,
            host,
            time_str,
            rtt_str,
            history_str
        );
    }

    println!();
}


