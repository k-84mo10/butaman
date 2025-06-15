# butaman
これはネットワーク監視システムです。

## 開発環境の作り方
- docker を一時的に起動して中に入る方法
`docker compose run rust-dev`
- docker を起動させる方法
`docker compose up (-d) (--build)`
- docker の中に入る方法
`docker exec -it rust-dev`

- webモードで動き出す方法
`cargo run -- --web`

## メモ
✅ 構成要素と役割
1. Ping送信モジュール（同期・非同期）
- tokio::process::Command（非同期）やstd::process::Command（同期）を使用
- OSごとに ping コマンドの仕様を切り替え
- 結果（成功・失敗、RTT、TTLなど）を PingResult 構造体に格納
- 別の端末にsshしてpingやtcp、icmpでのping、さらにはv6への対応も
2. ターミナルでの可視化
- crossterm や tui-rs を使用して、RTTバーやパケット損失率を表示
- 結果履歴は VecDeque に保持し、文字やグラフで描画
- タイトルやホスト一覧の描画、キーボードによる操作（rキーでリセットなど）
3. ログ出力
- std::fs::OpenOptions や tracing クレートでファイルに追記
- --logdir フラグ指定があれば保存、それ以外は標準出力
4. Web表示（オプション）
- warp や axum などのWebフレームワークでAPIを提供
- /status エンドポイントでJSON形式の状態を返す
- tokio::sync::RwLock 等で状態を共有