use serde::de::value;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use chrono::{DateTime, Utc, NaiveDateTime};
use serde_json::{Value, json};


fn timestamp_to_utc_string(timestamp: i64) -> String {
    let seconds = timestamp / 1000;
    let nanoseconds = (timestamp % 1000) * 1_000_000;

    let naive_datetime = NaiveDateTime::from_timestamp(seconds, nanoseconds as u32);
    let datetime_utc: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);

    datetime_utc.to_string()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Processing of monthly data
    let base_path = r#"C:\Users\adhuc\Desktop\rust-demo\test\jupyterlab\HISTORY\ALL\futures\um\monthly\klines"#;
    let output_path = r#"C:\Users\adhuc\Desktop\rust-demo\test\jupyterlab\HISTORY\ALL\futures\um\monthly\json_output"#;

    let mut coin_names = Vec::new();

    if let Ok(entries) = fs::read_dir(base_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let coin_name = entry.file_name().to_string_lossy().to_string();
                coin_names.push(coin_name);
            }
        }
    } else {
        println!("Failed to read directory");
    }

    for coin_name in &coin_names {
        let coin_k_files = format!(r#"{}\{}\1d\"#, base_path, coin_name);
        let json_file_path = format!(r#"{}\{}-1d-ALL.json"#, output_path, coin_name);

        let mut transformed_data_list = Vec::new();

        if let Ok(entries) = fs::read_dir(&coin_k_files) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let csv_path = format!(r#"{}\{}"#, coin_k_files, file_name.to_string_lossy());
                    dbg!(&csv_path);
                    let mut content = String::new();
                    if let Ok(mut file) = File::open(&csv_path) {
                        file.read_to_string(&mut content)?;
                    }

                    let lines: Vec<&str> = content.lines().collect();

                    for line in lines {
                        
                        let values: Vec<&str> = line.split(',').collect();
                        
                        if values.len() >= 12 {
                            if let Ok(timestamp) = values[0].parse::<i64>(){
                                                            // Assuming there are at least 12 values in a line
                            let open_time = timestamp_to_utc_string(values[0].parse::<i64>().unwrap());
                            let open = values[1].parse::<f64>().unwrap();
                            let high = values[2].parse::<f64>().unwrap();
                            let low = values[3].parse::<f64>().unwrap();
                            let close = values[4].parse::<f64>().unwrap();
                            let volume = values[5].parse::<f64>().unwrap();
                            let close_time = timestamp_to_utc_string(values[6].parse::<i64>().unwrap());
                            // let quote_volume = values[7].parse::<f64>().unwrap();
                            let trade_count = values[8].parse::<i64>().unwrap();
                            // let taker_buy_volume = values[9].parse::<f64>().unwrap();
                            // let taker_buy_quote_volume = values[10].parse::<f64>().unwrap();
                            // let ignore = values[11].parse::<i64>().unwrap();

                            let transformed_data = json!({
                                "start_time": open_time,
                                "close_time": close_time,
                                // "open_time": open_time,
                                "open": open,
                                "high": high,
                                "low": low,
                                "close": close,
                                "volume": volume,
                                
                                // "volume": quote_volume,
                                "trade_count": trade_count,
                                // "taker_buy_volume": taker_buy_volume,
                                // "taker_buy_quote_volume": taker_buy_quote_volume,
                                // "ignore": ignore,
                            });

                            transformed_data_list.push(transformed_data);
                            }

                        }
                    }
                }
            }
        } else {
            println!("Failed to read directory");
        }

        let json_str = serde_json::to_string_pretty(&transformed_data_list)?;

        let json_file_path = format!(r#"{}\{}-1d-ALL.json"#, output_path, coin_name);
        let mut json_file = File::create(json_file_path)?;
        json_file.write_all(json_str.as_bytes())?;
    }

    Ok(())
}


