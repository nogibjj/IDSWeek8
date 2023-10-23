use std::fs::File;
use std::time::Instant;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GameDetail {
    // Include all the columns from your CSV here. Assuming only PTS for simplicity
    PTS: i32,
}

fn main() {
    let start_time = Instant::now();

    let mut rdr = csv::Reader::from_path("games_details.csv").unwrap();
    let mut records: Vec<GameDetail> = rdr.deserialize().map(|res| res.unwrap()).collect();

    records.sort_by(|a, b| b.PTS.cmp(&a.PTS));

    // Optionally: Write sorted records to a new CSV file
    // let mut wtr = csv::Writer::from_path("sorted_games_details.csv").unwrap();
    // for record in records {
    //     wtr.serialize(record).unwrap();
    // }

    let elapsed = start_time.elapsed();
    println!("Execution Time: {:?}", elapsed);

    let memory_usage = get_memory_usage();
    println!("Memory Usage: {} MB", memory_usage / 1024);
}

#[cfg(target_os = "linux")]
fn get_memory_usage() -> u64 {
    use std::process::Command;

    let output = Command::new("ps")
        .arg("-o")
        .arg("rss=")
        .arg("-p")
        .arg(std::process::id().to_string())
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8_lossy(&output.stdout).trim().parse::<u64>().unwrap_or(0);
    result
}

