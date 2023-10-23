use std::time::Instant;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct GameDetail {
    // Include all the columns from your CSV here. Assuming only PTS for simplicity
    PTS: i32,
}

fn main() {
    let start_time = Instant::now();

    let mut rdr = match csv::Reader::from_path("/workspaces/IDSWeek8/games_details.csv") {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return;
        }
    };
    
    let mut records: Vec<GameDetail> = rdr.deserialize()
    .filter_map(Result::ok)  // Only keep successful results
    .collect();


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
    let cpu_usage = get_cpu_usage();
    println!("CPU Usage: {:.2}%", cpu_usage);

    
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


#[cfg(target_os = "linux")]
fn get_cpu_usage() -> f32 {
    use std::process::Command;

    let output = Command::new("ps")
        .arg("-p")
        .arg(std::process::id().to_string())
        .arg("-o")
        .arg("pcpu")
        .output()
        .expect("Failed to execute command");

    let result = String::from_utf8_lossy(&output.stdout);
    let usage: f32 = result.lines().last().unwrap_or("0.0").trim().parse().unwrap_or(0.0);

    usage
}
