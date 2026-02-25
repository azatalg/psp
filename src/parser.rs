use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn load_grades_data(
    filepath: &str,
) -> Result<(Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>), Box<dyn std::error::Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);

    let mut iq_b = Vec::new();
    let mut iq_g = Vec::new();
    let mut sroc_b = Vec::new();
    let mut sroc_g = Vec::new();
    let mut ph_b = Vec::new();
    let mut ph_g = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();


        if parts.is_empty() {
            continue;
        }

        if parts.len() >= 5 {

            let sroc: f64 = parts[1].parse().unwrap_or(0.0);
            let iq: f64 = parts[2].parse().unwrap_or(0.0);
            let gender = parts[3];
            let ph: f64 = parts[4].parse().unwrap_or(0.0);


            if gender.eq_ignore_ascii_case("m") {
                sroc_b.push(sroc);
                iq_b.push(iq);
                ph_b.push(ph);
            } else if gender.eq_ignore_ascii_case("f") {
                sroc_g.push(sroc);
                iq_g.push(iq);
                ph_g.push(ph);
            }
        }
    }

    Ok((iq_b, iq_g, sroc_b, sroc_g, ph_b, ph_g))
}