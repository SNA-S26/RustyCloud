use chrono::Utc;
use tokio::io::{AsyncWriteExt, stderr, stdout};

enum JsonLogStream {
    Stdout,
    Stderr,
}

async fn json_log(msg: &str, level: &str, log_stream: JsonLogStream) {
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let msg = msg.replace("\\", "\\\\").replace("\"", "\\\"");

    let log = format!(
        "{{\"timestamp\": \"{}\", \"level\": \"{}\", \"message\": \"{}\"}}\n",
        timestamp, level, msg
    );

    match log_stream {
        JsonLogStream::Stdout => {
            let _ = stdout().write_all(log.as_bytes()).await;
        }
        JsonLogStream::Stderr => {
            let _ = stderr().write_all(log.as_bytes()).await;
        }
    }
}

pub async fn info(msg: &str) {
    json_log(msg, "INFO", JsonLogStream::Stdout).await;
}

pub async fn warning(msg: &str) {
    json_log(msg, "WARNING", JsonLogStream::Stdout).await;
}

pub async fn error(msg: &str) {
    json_log(msg, "ERROR", JsonLogStream::Stderr).await;
}
