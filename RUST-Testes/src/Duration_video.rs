// pega a duração de um vídeo mo4

use std::process::Command;
use serde::Deserialize;

#[derive(Deserialize)]
struct Stream {
    duration: Option<String>,
}

#[derive(Deserialize)]
struct Streams {
    streams: Vec<Stream>,
}

fn with_ffprobe(filename: &str) -> f64 {
    let output = Command::new("ffprobe")
        .arg("-v")
        .arg("quiet")
        .arg("-show_streams")
        .arg("-select_streams")
        .arg("v:0")
        .arg("-of")
        .arg("json")
        .arg(filename)
        .output()
        .expect("Failed to execute ffprobe");

    if !output.status.success() {
        return 0.0;
    }

    let result = String::from_utf8_lossy(&output.stdout);

    match serde_json::from_str::<Streams>(&result) {
        Ok(streams) => {
            if let Some(stream) = streams.streams.get(0) {
                if let Some(duration_str) = &stream.duration {
                    duration_str.parse().unwrap_or(0.0)
                } else {
                    0.0
                }
            } else {
                0.0
            }
        }
        Err(_) => 0.0,
    }
}

fn main() {
    let filename = "example.mp4"; // Substitua pelo caminho do seu arquivo
    let duration = with_ffprobe(filename);
    println!("Duration: {}", duration);
  }
